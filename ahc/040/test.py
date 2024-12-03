import numpy as np
import matplotlib.pyplot as plt
import japanize_matplotlib  # noqa: F401

# パラメータ設定

# seed値の固定
np.random.seed(0)

# -- BEST CASE --
# N = 30  # 最小棒数
# T = 4 * N  # 最大計測回数
# sigma = 1000.0  # 計測誤差の標準偏差（最小）

# -- WORST CASE --
N = 100  # 最大棒数
T = N // 2  # 最小計測回数
sigma = 10000.0  # 計測誤差の標準偏差（最大）

# 真の棒の長さを生成（10000から50000の間の一様乱数）
lower_bound = 1e4
upper_bound = 5e4
true_lengths = np.random.uniform(lower_bound, upper_bound, N)

# 初期観測値（各棒の長さに計測誤差を加える）
initial_observations = true_lengths + np.random.normal(0, sigma, N)

# -- ここまで問題設定 --

qyery_calls = 0  # query関数の呼び出し回数
def query(indices):
    """計測アクションをエミュレートする関数"""
    global qyery_calls
    if qyery_calls >= T:
        raise ValueError("計測回数の上限を超えました")
    qyery_calls += 1
    return np.sum(true_lengths[indices]) + np.random.normal(0, sigma)


# 計測計画の作成
# 初期観測値（各棒の個別計測）は計測回数に含まれない
measurement_indices = []  # 各計測で使用された棒のインデックス
measurement_values = []  # 各計測で得られた観測値

# 初期観測値を推定に使用するために観測行列に追加
for i in range(N):
    measurement_indices.append([i])
    measurement_values.append(initial_observations[i])

# 追加の計測（T回分）、ランダムに棒の組み合わせを選んで計測
for _ in range(T):
    # # 棒の数をランダムに選択（2本以上）
    # num_rods = np.random.randint(2, N + 1)
    # N/2本の棒を選択
    num_rods = N // 2
    # 棒の組み合わせをランダムに選択
    rods = np.random.choice(N, num_rods, replace=False)
    measurement_indices.append(rods)
    # 選んだ棒の真の長さの合計に計測誤差を加える
    measurement_values.append(query(rods))

# 観測行列Aと観測ベクトルyを作成
M = len(measurement_values)  # 総計測回数（初期観測値 + 追加計測）
A = np.zeros((M, N))
for idx, rods in enumerate(measurement_indices):
    A[idx, rods] = 1  # 選ばれた棒の位置に1をセット

y = np.array(measurement_values)

# 最小二乗法による推定
# 事前分布の平均と分散を設定
# 棒の長さの平均値
prior_mean = (lower_bound + upper_bound) / 2
# 棒の長さの分散
prior_variance = (upper_bound - lower_bound) ** 2 / 12

# 事前分布を考慮した正則化項
lambda_reg = sigma**2 / prior_variance

AtA = A.T @ A + lambda_reg * np.eye(N)
AtY = A.T @ y + lambda_reg * prior_mean
estimated_lengths = np.linalg.solve(AtA, AtY)

# 初期観測値と最終推定値の誤差を計算
initial_errors = initial_observations - true_lengths
final_errors = estimated_lengths - true_lengths

# 初期2乗誤差と最終2乗誤差を計算
initial_mse = np.mean(initial_errors**2)
final_mse = np.mean(final_errors**2)

print(f"初期2乗誤差: {initial_mse:.2f}")
print(f"最終2乗誤差: {final_mse:.2f}")

# 結果を出力
print("各棒の初期観測値と最終推定値の誤差（上位10件）:")
# 誤差の絶対値が大きい順にソート
sorted_indices = np.argsort(abs(final_errors))[::-1]
for i in sorted_indices[:10]:
    print(
        f"棒{i + 1}: 初期誤差={initial_errors[i]:.2f}, 最終誤差={final_errors[i]:.2f}"
    )

# 初期誤差と最終誤差のヒストグラムを重ねて表示
plt.figure(figsize=(8, 5))

plt.hist(abs(initial_errors), bins=30, alpha=0.5, label="初期観測値の誤差", edgecolor="k")
plt.hist(abs(final_errors), bins=30, alpha=0.5, label="最終推定値の誤差", edgecolor="k")
plt.title("誤差分布の比較")
plt.xlabel("誤差 [単位長さ]")
plt.ylabel("頻度")
plt.legend()
plt.text(0.6, 0.9, f"初期MSE = {initial_mse:.2f}\n最終MSE = {final_mse:.2f}", transform=plt.gca().transAxes)

plt.tight_layout()
plt.show()

import numpy as np
import matplotlib.pyplot as plt
import japanize_matplotlib  # noqa: F401

# パラメータ設定

# seed値の固定
np.random.seed(0)

# -- BEST CASE --
# N = 30  # 最小タイル数
# T = 4 * N  # 最大計測回数
# sigma = 1000.0  # 計測誤差の標準偏差（最小）

# -- WORST CASE --
N = 100  # 最大タイル数
T = N // 2  # 最小計測回数
sigma = 10000.0  # 計測誤差の標準偏差（最大）

# 真のタイルの長さを生成（10000から50000の間の一様乱数）
lower_bound = 1e4
upper_bound = 5e4
true_width = np.random.uniform(lower_bound, upper_bound, N)
true_height = np.random.uniform(lower_bound, upper_bound, N)

# 初期観測値（各タイルの長さに計測誤差を加える）
initial_width_observations = true_width + np.random.normal(0, sigma, N)
initial_height_observations = true_height + np.random.normal(0, sigma, N)

# -- ここまで問題設定 --

qyery_calls = 0  # query関数の呼び出し回数


def query(x_indices, y_indices):
    """計測アクションをエミュレートする関数"""
    global qyery_calls
    if qyery_calls >= T:
        raise ValueError("計測回数の上限を超えました")
    qyery_calls += 1
    return np.sum(true_width[x_indices]) + np.random.normal(0, sigma), np.sum(
        true_height[y_indices]
    ) + np.random.normal(0, sigma)


# 計測計画の作成
# 初期観測値（各タイルの個別計測）は計測回数に含まれない
measurement_width_indices = []  # 各計測で使用されたタイルのインデックス (width)
measurement_width_values = []  # 各計測で得られた観測値 (width)
measurement_height_indices = []  # 各計測で使用されたタイルのインデックス (height)
measurement_height_values = []  # 各計測で得られた観測値 (height)

# 初期観測値を推定に使用するために観測行列に追加
for i in range(N):
    measurement_width_indices.append([i])
    measurement_width_values.append(initial_width_observations[i])
    measurement_height_indices.append([i])
    measurement_height_values.append(initial_height_observations[i])

# 追加の計測（T回分）、ランダムにタイルの組み合わせを選んで計測
for _ in range(T):
    # N/2個のタイルを選択
    num_tiles = N // 2 - 1
    # タイルの組み合わせをランダムに選択
    tiles = np.random.choice(N, num_tiles, replace=False)
    # get min and remove from tiles
    min_tile = np.argmin(true_width[tiles])
    tiles = np.delete(tiles, min_tile)
    # split 2 tiles to width group and height group randomly
    np.random.shuffle(tiles)
    width_tiles = tiles[:num_tiles // 2]
    height_tiles = tiles[num_tiles // 2:]
    # sort
    width_tiles = np.sort(width_tiles)
    height_tiles = np.sort(height_tiles)
    # prepend min_tile
    width_tiles = np.insert(width_tiles, 0, min_tile)
    height_tiles = np.insert(height_tiles, 0, min_tile)
    # 計測アクションを実行
    width, height = query(width_tiles, height_tiles)
    # 計測結果を保存
    measurement_width_indices.append(width_tiles)
    measurement_width_values.append(width)
    measurement_height_indices.append(height_tiles)
    measurement_height_values.append(height)

# 観測行列Aと観測ベクトルyを作成
M = len(measurement_width_values)  # 総計測回数（初期観測値 + 追加計測）
A_width = np.zeros((M, N))
A_height = np.zeros((M, N))
for idx, rods in enumerate(measurement_width_indices):
    A_width[idx, rods] = 1  # 選ばれたタイルの位置に1をセット
for idx, rods in enumerate(measurement_height_indices):
    A_height[idx, rods] = 1  # 選ばれたタイルの位置に1をセット

y_width = np.array(measurement_width_values)
y_height = np.array(measurement_height_values)

# 最小二乗法による推定
# 事前分布の平均と分散を設定
# タイルの長さの平均値
prior_mean = (lower_bound + upper_bound) / 2
# タイルの長さの分散
prior_variance = (upper_bound - lower_bound) ** 2 / 12

# 事前分布を考慮した正則化項
lambda_reg = sigma**2 / prior_variance

AtA_width = A_width.T @ A_width + lambda_reg * np.eye(N)
AtY_width = A_width.T @ y_width + lambda_reg * prior_mean
estimated_widths = np.linalg.solve(AtA_width, AtY_width)

AtA_height = A_height.T @ A_height + lambda_reg * np.eye(N)
AtY_height = A_height.T @ y_height + lambda_reg * prior_mean
estimated_heights = np.linalg.solve(AtA_height, AtY_height)

# 初期観測値と最終推定値の誤差を計算
initial_width_errors = initial_width_observations - true_width
final_width_errors = estimated_widths - true_width
initial_height_errors = initial_height_observations - true_height
final_height_errors = estimated_heights - true_height

# 初期2乗誤差と最終2乗誤差を計算
initial_width_mse = np.mean(initial_width_errors**2)
final_width_mse = np.mean(final_width_errors**2)
initial_height_mse = np.mean(initial_height_errors**2)
final_height_mse = np.mean(final_height_errors**2)

print(f"初期2乗誤差 (幅): {initial_width_mse:.2f}")
print(f"最終2乗誤差 (幅): {final_width_mse:.2f}")
print(f"初期2乗誤差 (高さ): {initial_height_mse:.2f}")
print(f"最終2乗誤差 (高さ): {final_height_mse:.2f}")

# 結果を出力
print("各タイルの初期観測値と最終推定値の誤差（上位10件）:")
# 誤差の絶対値が大きい順にソート
sorted_width_indices = np.argsort(abs(final_width_errors))[::-1]
sorted_height_indices = np.argsort(abs(final_height_errors))[::-1]
for i in sorted_width_indices[:10]:
    print(
        f"タイル{i + 1} (幅): 初期誤差={initial_width_errors[i]:.2f}, 最終誤差={final_width_errors[i]:.2f}"
    )
print("...")
for i in sorted_width_indices[-10:]:
    print(
        f"タイル{i + 1} (幅): 初期誤差={initial_width_errors[i]:.2f}, 最終誤差={final_width_errors[i]:.2f}"
    )
print()
for i in sorted_height_indices[:10]:
    print(
        f"タイル{i + 1} (高さ): 初期誤差={initial_height_errors[i]:.2f}, 最終誤差={final_height_errors[i]:.2f}"
    )
print("...")
for i in sorted_height_indices[-10:]:
    print(
        f"タイル{i + 1} (高さ): 初期誤差={initial_height_errors[i]:.2f}, 最終誤差={final_height_errors[i]:.2f}"
    )

fig, axes = plt.subplots(1, 2, figsize=(16, 5))

# 初期誤差と最終誤差のヒストグラムを重ねて表示 (幅)
axes[0].hist(
  abs(initial_width_errors), bins=30, alpha=0.5, label="初期観測値の誤差 (幅)", edgecolor="k"
)
axes[0].hist(abs(final_width_errors), bins=30, alpha=0.5, label="最終推定値の誤差 (幅)", edgecolor="k")
axes[0].set_title("誤差分布の比較 (幅)")
axes[0].set_xlabel("誤差 [単位長さ]")
axes[0].set_ylabel("頻度")
axes[0].legend()
axes[0].text(
  0.6,
  0.9,
  f"初期MSE (幅) = {initial_width_mse:.2f}\n最終MSE (幅) = {final_width_mse:.2f}",
  transform=axes[0].transAxes,
)

# 初期誤差と最終誤差のヒストグラムを重ねて表示 (高さ)
axes[1].hist(
  abs(initial_height_errors), bins=30, alpha=0.5, label="初期観測値の誤差 (高さ)", edgecolor="k"
)
axes[1].hist(abs(final_height_errors), bins=30, alpha=0.5, label="最終推定値の誤差 (高さ)", edgecolor="k")
axes[1].set_title("誤差分布の比較 (高さ)")
axes[1].set_xlabel("誤差 [単位長さ]")
axes[1].set_ylabel("頻度")
axes[1].legend()
axes[1].text(
  0.6,
  0.9,
  f"初期MSE (高さ) = {initial_height_mse:.2f}\n最終MSE (高さ) = {final_height_mse:.2f}",
  transform=axes[1].transAxes,
)

plt.tight_layout()
plt.show()

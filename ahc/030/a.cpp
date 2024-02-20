#include <bits/stdc++.h>
using namespace std;

#define rep(i, a, b) for (int i = a; i < (int)b; i++)

template <class T, class S>
bool chmax(T &a, const S &b) {
    if (a < (T)b) {
        a = (T)b;
        return 1;
    }
    return 0;
}
template <class T, class S>
bool chmin(T &a, const S &b) {
    if ((T)b < a) {
        a = (T)b;
        return 1;
    }
    return 0;
}

// 乱数生成器
struct RandomNumberGenerator {
    mt19937 mt;

    RandomNumberGenerator()
        : mt(chrono::steady_clock::now().time_since_epoch().count()) {}

    int operator()(int a, int b) {  // [a, b)
        uniform_int_distribution<int> dist(a, b - 1);
        return dist(mt);
    }

    int operator()(int b) {  // [0, b)
        return (*this)(0, b);
    }
} rnd;

// 正規分布の累積分布関数
constexpr double normal_cdf(double x, double mean = 0.0, double sigma = 1.0) {
    return 0.5 * (1.0 + std::erf((x - mean) / (sigma * 1.41421356237)));
}
constexpr double probability_in_range(double l, double r, double mean = 0.0,
                                      double sigma = 1.0) {
    assert(l <= r);
    if (mean < l)
        return probability_in_range(2.0 * mean - r, 2.0 * mean - l, mean,
                                    sigma);
    double p_l = normal_cdf(l, mean, sigma);
    double p_r = normal_cdf(r, mean, sigma);
    return p_r - p_l;
}

void normalize(vector<double> &v) {
    double s = 0;
    for (auto d : v) {
        s += d;
    }
    assert(s > 0);

    for (auto &d : v) {
        d /= s;
    }
}

// 質問query
int query(vector<pair<int, int>> v) {
    cout << "q"
         << " " << v.size() << ' ';
    for (auto [x, y] : v) {
        cout << x << ' ' << y << ' ';
    }

    cout << endl;

    int res;
    cin >> res;
    return res;
};

// 解答query
int answer(vector<pair<int, int>> v) {
    cout << "a"
         << " " << v.size() << ' ';
    for (auto [x, y] : v) {
        cout << x << ' ' << y << ' ';
    }

    cout << endl;

    int res;
    cin >> res;
    return res;
}

int N, M;
double EPS;
vector<vector<pair<int, int>>> V;
vector<int> x_max;
vector<int> y_max;

//  入力を受け取る。ついでに各ポリオミノの縦横の大きさも
void input() {
    cin >> N >> M >> EPS;
    V.resize(M);
    x_max.resize(M);
    y_max.resize(M);

    rep(i, 0, M) {
        int k;
        cin >> k;
        V[i].resize(k);
        rep(j, 0, k) {
            int x, y;
            cin >> x >> y;
            V[i][j] = {x, y};
            chmax(x_max[i], x);
            chmax(y_max[i], y);
        }
    }
}

// 個数k, v(S) = cntの点集合からresが得られたときの尤度(条件付き確率)
double lilelihood(int k, int cnt, int res) {
    double mean = (k - cnt) * EPS + cnt * (1 - EPS);
    double sigma = sqrt(k * EPS * (1 - EPS));

    if (res == 0) return probability_in_range(-1e10, res + 0.5, mean, sigma);
    return probability_in_range(res - 0.5, res + 0.5, mean, sigma);
}

// bfsで可能なすべての盤面を生成。Mが大きいときはTLE
vector<vector<vector<int>>> make_all_candidates() {
    vector<vector<vector<int>>> q;
    q.push_back(vector(N, vector(N, 0)));

    rep(i, 0, M) {
        vector<vector<vector<int>>> nq;
        for (auto b : q) {
            rep(j, 0, N - x_max[i]) rep(k, 0, N - y_max[i]) {
                auto nb = b;
                for (auto [x, y] : V[i]) {
                    nb[j + x][y + k]++;
                }
                nq.push_back(nb);
            }
        }

        swap(nq, q);
    }

    return q;
}

// ベイズ推定をする
void bayesian_inference(vector<vector<vector<int>>> candidates) {
    int n = candidates.size();

    // 初期確率は全て等しい
    vector<double> p(n, 1.0 / n);

    string scan_mode = "vertical";  // "horizontal" or "random"
    int scan_index = 0;
    int scan_width = (N + 1) / 2;

    while (1) {
        set<pair<int, int>> st;
        int k = 0;

        // randomに点を20個選ぶ
        k = 20;
        while (st.size() < k) {
            int i = rnd(N);
            int j = rnd(N);
            st.insert({i, j});
        }

        vector<pair<int, int>> v;
        for (auto p : st) v.push_back(p);

        // 点集合についてqueryを投げる
        int res = query(v);

        rep(i, 0, n) {
            // i番目の盤面を仮定したときのv(S)の値を取得
            int cnt = 0;
            for (auto [x, y] : v) {
                cnt += candidates[i][x][y];
            }

            // 事前確率に尤度を掛ける
            p[i] *= lilelihood(k, cnt, res);
        }

        normalize(p);

        auto max_it = max_element(p.begin(), p.end());
        int max_ind = max_it - p.begin();

        // 80%以上の確率で正解できそうなら聞いてみる
        if (*max_it > 0.8) {
            vector<pair<int, int>> v;
            rep(i, 0, N) rep(j, 0, N) {
                if (candidates[max_ind][i][j] > 0) v.push_back({i, j});
            }

            // 正解なら終了。だめならその事前確率を0にして継続
            if (answer(v))
                break;
            else
                p[max_ind] = 0.0;
        }
    }
}

int main() {
    input();

    bayesian_inference(make_all_candidates());
}

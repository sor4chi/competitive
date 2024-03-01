#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
template <class T, class... Ts>
void print(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
}
template <class T, class... Ts>
void println(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
    cout << '\n';
}
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void println() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

unsigned long rng() {
    static unsigned long x = 88172645463325252UL;
    x ^= x << 7;
    x ^= x >> 9;
    return x;
}

double rnd() {
    return (double)rng() / ULONG_MAX;
}

unsigned long lpow(unsigned long x, unsigned long n) {
    unsigned long ret = 1;
    while (n > 0) {
        if (n & 1) ret *= x;
        x *= x;
        n >>= 1;
    }
    return ret;
}

int N, K;
int a[10];
int R = 1e4;
int MOVE_RANGE = 300;
vector<pair<int, int>> strawberries;
chrono::system_clock::time_point start;
chrono::milliseconds time_limit(1980);

double start_temp = 1500;
double end_temp = 100;

// 近傍
enum class Neighbor {
    X_MOVE,
    Y_MOVE,
    X_DECREASE,
    X_INCREASE,
    Y_DECREASE,
    Y_INCREASE,
};

struct Solver {
    vector<int> best_x_splits, best_y_splits;

    void answer() {
        println(best_x_splits.size() + best_y_splits.size());
        for (int x : best_x_splits) {
            println(x, -R, x, R);
        }
        for (int y : best_y_splits) {
            println(-R, y, R, y);
        }
    }

    map<pair<int, int>, int> cnt;
    int evaluate(vector<int>& x_splits, vector<int>& y_splits) {
        cnt.clear();
        rep(i, N) {
            int x = strawberries[i].first;
            int y = strawberries[i].second;
            int x_idx = lower_bound(x_splits.begin(), x_splits.end(), x) - x_splits.begin();
            int y_idx = lower_bound(y_splits.begin(), y_splits.end(), y) - y_splits.begin();
            // 線上にあれば除外
            if (x_idx < x_splits.size() && x_splits[x_idx] == x) continue;
            if (y_idx < y_splits.size() && y_splits[y_idx] == y) continue;
            cnt[{x_idx, y_idx}]++;
        }
        map<int, int> b;
        for (auto [k, v] : cnt) {
            b[v]++;
        }
        int up = 0;
        int down = 0;
        rep(i, 10) {
            up += min(b[i + 1], a[i]);
            down += a[i];
        }
        return (int)round(1e6 * up / down);
    }

    void solve() {
        int halfK = K / 2;
        rep(i, halfK) {
            int x = rng() % (2 * R) - R;
            int y = rng() % (2 * R) - R;
            best_x_splits.push_back(x);
            best_y_splits.push_back(y);
        }
        sort(best_x_splits.begin(), best_x_splits.end());
        sort(best_y_splits.begin(), best_y_splits.end());
        int best_score = 0;
        int iter = 0;
        while (chrono::system_clock::now() - start < time_limit) {
            double temp = start_temp + (end_temp - start_temp) * (chrono::system_clock::now() - start) / time_limit;
            iter++;
            Neighbor selected = Neighbor(rng() % 6);
            vector<int> x_splits = best_x_splits;
            vector<int> y_splits = best_y_splits;
            double score = 0;
            if (selected == Neighbor::X_DECREASE) {
                if (x_splits.size() > 1) {
                    int idx = rng() % (x_splits.size() - 1);
                    x_splits.erase(x_splits.begin() + idx);
                    score = evaluate(x_splits, y_splits);
                } else {
                    continue;
                }
            }
            if (selected == Neighbor::X_INCREASE) {
                if (x_splits.size() < halfK) {
                    int idx = rng() % x_splits.size();
                    int x = rng() % (2 * R) - R;
                    x_splits.insert(x_splits.begin() + idx, x);
                    sort(x_splits.begin(), x_splits.end());
                    score = evaluate(x_splits, y_splits);
                } else {
                    continue;
                }
            }
            if (selected == Neighbor::Y_DECREASE) {
                if (y_splits.size() > 1) {
                    int idx = rng() % (y_splits.size() - 1);
                    y_splits.erase(y_splits.begin() + idx);
                    score = evaluate(x_splits, y_splits);
                } else {
                    continue;
                }
            }
            if (selected == Neighbor::Y_INCREASE) {
                if (y_splits.size() < halfK) {
                    int idx = rng() % y_splits.size();
                    int y = rng() % (2 * R) - R;
                    y_splits.insert(y_splits.begin() + idx, y);
                    sort(y_splits.begin(), y_splits.end());
                    score = evaluate(x_splits, y_splits);
                } else {
                    continue;
                }
            }
            if (selected == Neighbor::X_MOVE) {
                int idx = rng() % x_splits.size();
                int x = rng() % (2 * R) - R;
                x_splits[idx] = x;
                sort(x_splits.begin(), x_splits.end());
                score = evaluate(x_splits, y_splits);
            }
            if (selected == Neighbor::Y_MOVE) {
                int idx = rng() % y_splits.size();
                int y = rng() % (2 * R) - R;
                y_splits[idx] = y;
                sort(y_splits.begin(), y_splits.end());
                score = evaluate(x_splits, y_splits);
            }

            int diff = score - best_score;
            if (diff > 0 || exp(diff / temp) > rnd()) {
                best_score = score;
                best_x_splits = x_splits;
                best_y_splits = y_splits;
                answer();
            }
        }
        cerr << "iter: " << iter << endl;
    }
};

int main(int argc, char* argv[]) {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    bool is_arg_contain_export = false;
    for (int i = 0; i < argc; i++) {
        if (string(argv[i]) == "--export") {
            is_arg_contain_export = true;
            break;
        }
    }

    start = chrono::system_clock::now();
    input(N, K);
    K *= 0.8;
    rep(i, 10) input(a[i]);
    rep(i, N) {
        int x, y;
        input(x, y);
        strawberries.push_back({x, y});
    }

    Solver s;
    s.solve();
    if (is_arg_contain_export)
        println("Score =", s.evaluate(s.best_x_splits, s.best_y_splits));

    return 0;
}

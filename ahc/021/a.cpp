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

int PYRAMID_SIZE = 30;
int TOTAL = PYRAMID_SIZE * (PYRAMID_SIZE + 1) / 2;
int MAX_ITER = 10000;

typedef pair<int, int> Coordinate;

struct Ball {
    int value;
};

struct Swap {
    Coordinate a;
    Coordinate b;
};

struct Solver {
    map<Coordinate, Ball> pyramid;
    vector<Coordinate> smalls = vector<Coordinate>(TOTAL);
    vector<Swap> swaps;

    Solver(map<Coordinate, Ball> pyramid, vector<Swap> swaps) {
        this->pyramid = pyramid;
        this->swaps = swaps;
        init();
    }

    void init() {
        for (auto [coordinate, ball] : pyramid) {
            smalls[ball.value] = coordinate;
        }
    }

    int get_index(Coordinate coordinate) {
        return coordinate.first * (coordinate.first + 1) / 2 + coordinate.second;
    }

    void solve() {
        for (auto coordinate : smalls) {
            while (coordinate.first > 0) {
                pair<int, int> lt = {coordinate.first - 1, coordinate.second - 1};
                pair<int, int> rt = {coordinate.first - 1, coordinate.second};
                int value = pyramid[coordinate].value;
                if (coordinate.second == 0) {
                    int rtv = pyramid[rt].value;
                    if (rtv > value) {
                        swaps.push_back({coordinate, rt});
                        swap(pyramid[coordinate], pyramid[rt]);
                        // update smalls
                        smalls[value] = rt;
                        smalls[rtv] = coordinate;
                        coordinate = rt;
                        continue;
                    } else {
                        break;
                    }
                }
                if (coordinate.second == coordinate.first) {
                    int ltv = pyramid[lt].value;
                    if (ltv > value) {
                        swaps.push_back({coordinate, lt});
                        swap(pyramid[coordinate], pyramid[lt]);
                        // update smalls
                        smalls[value] = lt;
                        smalls[ltv] = coordinate;
                        coordinate = lt;
                        continue;
                    } else {
                        break;
                    }
                }
                int ltv = pyramid[lt].value;
                int rtv = pyramid[rt].value;
                if (ltv > value && rtv > value) {
                    if (ltv > rtv) {
                        swaps.push_back({coordinate, lt});
                        swap(pyramid[coordinate], pyramid[lt]);
                        // update smalls
                        smalls[value] = lt;
                        smalls[ltv] = coordinate;
                        coordinate = lt;
                        continue;
                    } else {
                        swaps.push_back({coordinate, rt});
                        swap(pyramid[coordinate], pyramid[rt]);
                        // update smalls
                        smalls[value] = rt;
                        smalls[rtv] = coordinate;
                        coordinate = rt;
                        continue;
                    }
                } else if (ltv > value) {
                    swaps.push_back({coordinate, lt});
                    swap(pyramid[coordinate], pyramid[lt]);
                    // update smalls
                    smalls[value] = lt;
                    smalls[ltv] = coordinate;
                    coordinate = lt;
                    continue;
                } else if (rtv > value) {
                    swaps.push_back({coordinate, rt});
                    swap(pyramid[coordinate], pyramid[rt]);
                    // update smalls
                    smalls[value] = rt;
                    smalls[rtv] = coordinate;
                    coordinate = rt;
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    void answer() {
        println(swaps.size());
        for (auto swap : swaps) {
            println(swap.a.first, swap.a.second, swap.b.first, swap.b.second);
        }
    }
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    map<Coordinate, Ball> pyramid;
    vector<Swap> swaps;

    rep(i, PYRAMID_SIZE) {
        rep(j, i + 1) {
            int value;
            input(value);
            pyramid[{i, j}] = {value};
        }
    }

    Solver s = Solver(pyramid, swaps);
    s.solve();
    s.answer();

    return 0;
}

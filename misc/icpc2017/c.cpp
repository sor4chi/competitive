#include <bits/stdc++.h>

// #include <atcoder/all>

using namespace std;
// using namespace atcoder;
typedef long long ll;
template <class T, class... Ts>
void println(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
    cout << '\n';
}
template <class T>
void printv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cout << x << sep);
    }
    cout << end;
}
void println() { cout << '\n'; }
template <class T, class... Ts>
void eprintln(const T& a, const Ts&... b) {
    cerr << a;
    (void)(cerr << ... << (cerr << ' ', b));
    cerr << '\n';
}
template <class T>
void eprintv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cerr << x << sep);
    }
    cerr << end;
}
void eprintln() { cerr << '\n'; }
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    while (1) {
        ll n, m;
        input(n, m);
        if (n == 0 && m == 0) break;

        vector<vector<ll>> board(n, vector<ll>(m));
        rep(i, n) rep(j, m) input(board[i][j]);

        // rep(sx, n - 2) {
        ll best_ans = 0;
        for (ll sx = 0; sx < n - 2; sx++) {
            for (ll ex = sx + 2; ex < n; ex++) {
                for (ll sy = 0; sy < m - 2; sy++) {
                    for (ll ey = sy + 2; ey < m; ey++) {
                        // println("RANGE", sx, ex, sy, ey);
                        ll maxv = 0;
                        ll minv = LLONG_MAX;
                        set<ll> outer, inner;
                        vector<ll> inner_all;
                        for (ll x = sx; x <= ex; x++) {
                            for (ll y = sy; y <= ey; y++) {
                                // println(x, y);
                                if ((x == sx || x == ex) || (y == sy || y == ey)) {
                                    // println("outer", x, y);
                                    outer.insert(board[x][y]);
                                } else {
                                    // println("inner", x, y);
                                    inner.insert(board[x][y]);
                                    inner_all.push_back(board[x][y]);
                                }
                            }
                        }
                        // printv(outer);
                        // printv(inner);
                        ll lower_outer = *outer.begin();
                        ll upper_inner = *inner.rbegin();
                        // println(lower_outer, upper_inner);
                        if (lower_outer > upper_inner) {
                            ll tot = 0;
                            for (auto i : inner_all) {
                                tot += (lower_outer - i);
                            }
                            best_ans = max(best_ans, tot);
                        }
                    }
                }
            }
        }
        println(best_ans);
    }

    return 0;
}

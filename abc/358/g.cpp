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
template <class T>
void printv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cout << x << sep);
    }
    cout << end;
}
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void println() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll h, w, k;
    input(h, w, k);
    ll sx, sy;
    input(sx, sy);
    vector<vector<int>> a(h, vector<int>(w));
    rep(i, h) rep(j, w) input(a[i][j]);

    vector<vector<vector<ll>>> dp(h * w + 1, vector<vector<ll>>(h, vector<ll>(w, -1)));  // dp[turn][x][y]: turn回目に(x, y)にいるときの最大スコア
    dp[0][sx - 1][sy - 1] = 0;

    vector<pair<int8_t, int8_t>> directions = {
        {0, 1},
        {0, -1},
        {1, 0},
        {-1, 0},
        {0, 0},
    };

    rep(turn, min(h * w, k)) {
        rep(x, h) {
            rep(y, w) {
                if (dp[turn][x][y] == -1) {
                    continue;
                }
                for (auto [dx, dy] : directions) {
                    ll nx = x + dx;
                    ll ny = y + dy;
                    if (nx < 0 || nx >= h || ny < 0 || ny >= w) {
                        continue;
                    }
                    dp[turn + 1][nx][ny] = max(dp[turn + 1][nx][ny], dp[turn][x][y] + a[nx][ny]);
                }
            }
        }
    }

    ll last = min(h * w, k);
    ll ans = 0;
    rep(x, h) {
        rep(y, w) {
            ans = max(ans, dp[last][x][y] + a[x][y] * (k - last));
        }
    }

    println(ans);

    return 0;
}

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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll N;
    input(N);
    vector<vector<ll>> jobs(N, vector<ll>(3, 0));
    rep(i, N) input(jobs[i][0], jobs[i][1], jobs[i][2]);
    vector<vector<ll>> dp(N + 1, vector<ll>(3, 0));
    rep(i, 3) dp[0][i] = 0;

    rep(i, N) {
        rep(j, 3) {
            rep(k, 3) {
                if (j == k) continue;
                dp[i + 1][k] = max(dp[i + 1][k], dp[i][j] + jobs[i][k]);
            }
        }
    }

    ll ans = 0;
    rep(i, 3) ans = max(ans, dp[N][i]);
    println(ans);

    return 0;
}

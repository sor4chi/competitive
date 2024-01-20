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

    ll N, W;
    input(N, W);
    vector<pair<ll, ll>> wv(N);
    rep(i, N) {
        ll w, v;
        input(w, v);
        wv[i] = {w, v};
    }

    vector<vector<ll>> dp(N + 1, vector<ll>(W + 1, 0));
    rep(i, N) {
        rep(j, W + 1) {
            if (j - wv[i].first >= 0) {
                dp[i + 1][j] = max(dp[i][j], dp[i][j - wv[i].first] + wv[i].second);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    println(dp[N][W]);

    return 0;
}

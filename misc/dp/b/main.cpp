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

    ll N, K;
    input(N, K);
    vector<ll> h(N), dp(N + K, 1e9);
    rep(i, N) input(h[i]);


    dp[0] = 0;

    rep(i, N) {
        rep1(j, K) {
            dp[i + j] = min(dp[i + j], abs(h[i] - h[i + j]) + dp[i]);
        }
    }

    println(dp[N - 1]);

    return 0;
}

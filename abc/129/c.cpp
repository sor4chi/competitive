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

const ll MOD = 1000000007;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, m;
    input(n, m);
    set<ll> a;
    rep(i, m) {
        ll ai;
        input(ai);
        a.insert(ai);
    }
    // dp[i] = i にたどり着くまでの組み合わせの数
    vector<ll> dp(n + 2, 0);
    dp[0] = 1;
    rep(i, n) {
        dp[i + 1] += a.find(i + 1) != a.end() ? 0 : dp[i];
        dp[i + 2] += a.find(i + 2) != a.end() ? 0 : dp[i];
        dp[i + 1] %= MOD;
        dp[i + 2] %= MOD;
    }
    println(dp[n]);

    return 0;
}

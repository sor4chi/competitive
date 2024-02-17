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

ll gcd(ll a, ll b) {
    if (b == 0) {
        return a;
    }
    return gcd(b, a % b);
}

ll lcm(ll a, ll b) {
    return a / gcd(a, b) * b;
}

// A以下の正の整数のうち、NかMのどちらか一方のみで割り切れるものの数を求める
ll count_divisible_by_N(ll A, ll N, ll M) {
    // Nで割り切れるものの数
    ll n = A / N;
    // Mで割り切れるものの数
    ll m = A / M;
    // NでもMでも割り切れるものの数
    ll nm = A / lcm(N, M);
    // Nだけで割り切れるものの数
    ll n_only = n - nm;
    // Mだけで割り切れるものの数
    ll m_only = m - nm;
    // NかMのどちらか一方のみで割り切れるものの数
    ll n_or_m_only = n_only + m_only;
    return n_or_m_only;
}

int main() {
    ll N, M, K;
    input(N, M, K);

    // NかMのどちらか一方のみで割り切れるものの数のうち、小さい順にK番目のものを求める
    ll left = 0;
    ll right = 1e18;
    while (right - left > 1) {
        ll mid = (left + right) / 2;
        if (count_divisible_by_N(mid, N, M) >= K) {
            right = mid;
        } else {
            left = mid;
        }
    }
    println(right);

    return 0;
}

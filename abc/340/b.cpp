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

    ll n;
    input(n);
    vector<ll> a(n);
    rep(i, n) input(a[i]);
    vector<pair<ll, ll>> st(n);
    rep(i, n - 1) {
        ll s, t;
        input(s, t);
        st[i] = make_pair(s, t);
    }

    rep(i, n - 1) {
        ll s = a[i] / st[i].first;
        a[i + 1] += st[i].second * s;
    }
    println(a[n - 1]);

    return 0;
}

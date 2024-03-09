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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n;
    input(n);
    vector<ll> a(n);
    rep(i, n) { input(a[i]); }
    ll m;
    input(m);
    vector<ll> b(m);
    rep(i, m) { input(b[i]); }
    ll l;
    input(l);
    vector<ll> c(l);
    rep(i, l) { input(c[i]); }

    set<ll> cache;
    rep(i, n) {
        rep(j, m) {
            rep(k, l) {
                cache.insert(a[i] + b[j] + c[k]);
            }
        }
    }

    ll q;
    input(q);
    rep(i, q) {
        ll x;
        input(x);
        yesno(cache.count(x));
    }

    return 0;
}

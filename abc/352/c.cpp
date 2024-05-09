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

    ll n;
    input(n);
    vector<ll> a(n), b(n);
    rep(i, n) input(a[i], b[i]);

    // aの累積和
    vector<ll> sa(n + 1);
    sa[0] = 0;
    rep(i, n) sa[i + 1] = sa[i] + a[i];
    // i以外のaの和を求める
    ll tmax = 0;
    rep(i, n) {
        // i以下のaの和
        ll lower_a = sa[i];
        // iより大きいaの和
        ll upper_a = sa[n] - sa[i + 1];
        // i
        ll t = lower_a + upper_a + b[i];
        tmax = max(tmax, t);
    }

    println(tmax);

    return 0;
}

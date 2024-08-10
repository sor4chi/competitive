#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
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

    ll n;
    input(n);
    vector<vector<vector<ll>>> a(n + 1, vector<vector<ll>>(n + 1, vector<ll>(n + 1)));
    rep1(i, n) rep1(j, n) rep1(k, n) {
        input(a[i][j][k]);
    }

    // 3次元累積和
    vector<vector<vector<ll>>> s(n + 1, vector<vector<ll>>(n + 1, vector<ll>(n + 1)));
    rep1(i, n) rep1(j, n) rep1(k, n) {
        s[i][j][k] = a[i][j][k] + s[i - 1][j][k] + s[i][j - 1][k] + s[i][j][k - 1] - s[i - 1][j - 1][k] - s[i - 1][j][k - 1] - s[i][j - 1][k - 1] + s[i - 1][j - 1][k - 1];
    }

    ll q;
    input(q);
    while (q--) {
        ll lx, rx, ly, ry, lz, rz;
        input(lx, rx, ly, ry, lz, rz);
        ll ans = s[rx][ry][rz] - s[lx - 1][ry][rz] - s[rx][ly - 1][rz] - s[rx][ry][lz - 1] + s[lx - 1][ly - 1][rz] + s[lx - 1][ry][lz - 1] + s[rx][ly - 1][lz - 1] - s[lx - 1][ly - 1][lz - 1];
        println(ans);
    }

    return 0;
}

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

    pair<ll, ll> xa, xb, xc;
    input(xa.first, xa.second, xb.first, xb.second, xc.first, xc.second);

    // a中心の時
    if ((xb.first - xa.first) * (xc.first - xa.first) + (xb.second - xa.second) * (xc.second - xa.second) == 0) {
        println("Yes");
        return 0;
    }

    // b中心の時
    if ((xc.first - xb.first) * (xa.first - xb.first) + (xc.second - xb.second) * (xa.second - xb.second) == 0) {
        println("Yes");
        return 0;
    }

    // c中心の時
    if ((xa.first - xc.first) * (xb.first - xc.first) + (xa.second - xc.second) * (xb.second - xc.second) == 0) {
        println("Yes");
        return 0;
    }

    println("No");

    return 0;
}

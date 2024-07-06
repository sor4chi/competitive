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

bool is_in(ll min1, ll max1, ll min2, ll max2) {
    return !(max1 <= min2 || max2 <= min1);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll a, b, c, d, e, f, g, h, i, j, k, l;
    input(a, b, c, d, e, f, g, h, i, j, k, l);

    if (a > d) swap(a, d);
    if (b > e) swap(b, e);
    if (c > f) swap(c, f);

    if (g > j) swap(g, j);
    if (h > k) swap(h, k);
    if (i > l) swap(i, l);

    bool overlapX = is_in(a, d, g, j);
    bool overlapY = is_in(b, e, h, k);
    bool overlapZ = is_in(c, f, i, l);

    yesno(overlapX && overlapY && overlapZ);

    return 0;
}

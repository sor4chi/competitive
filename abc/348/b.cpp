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
    vector<pair<ll, ll>> a(n);
    rep(i, n) {
        ll x, y;
        input(x, y);
        a[i] = {x, y};
    }

    // 点iからいちばん遠い点を探す
    rep(i, n) {
        ll max_dist = 0;
        ll max_index = -1;
        rep(j, n) {
            if (i == j) {
                continue;
            }
            ll dx = a[i].first - a[j].first;
            ll dy = a[i].second - a[j].second;
            if (max_dist < dx * dx + dy * dy) {
                max_dist = dx * dx + dy * dy;
                max_index = j;
            }
        }
        println(max_index + 1);
    }

    return 0;
}

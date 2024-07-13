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
    vector<pair<ll, ll>> lr(n);
    rep(i, n) input(lr[i].first, lr[i].second);

    // 最低値を全て取った場合と最大値を全て取った場合を考える
    ll minv = 0, maxv = 0;
    rep(i, n) {
        minv += lr[i].first;
        maxv += lr[i].second;
    }

    if (minv <= 0 && 0 <= maxv) {
        println("Yes");

        vector<ll> result(n);

        // 最低値で初期化
        rep(i, n) {
            result[i] = lr[i].first;
        }

        ll required = -minv;

        // 0になるまで必要な分を引いていく
        rep(i, n) {
            ll add = min(required, lr[i].second - lr[i].first);
            result[i] += add;
            required -= add;
            if (required == 0) break;
        }

        printv(result);
    } else {
        println("No");
    }

    return 0;
}

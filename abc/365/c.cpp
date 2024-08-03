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

    ll n, m;
    input(n, m);
    vector<ll> a(n);
    rep(i, n) input(a[i]);

    // カバーできるかどうかを判定する関数
    auto isCoverable = [&](ll cost) {
        ll tot = 0;
        rep(i, n) {
            tot += min(a[i], cost);
        }
        return tot <= m;
    };

    // カバーできる最大のコストをにぶたん
    ll left = 0, right = *max_element(a.begin(), a.end());
    while (left <= right) {
        ll mid = left + (right - left) / 2;
        if (isCoverable(mid)) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    // 最大値を求めておく
    ll maxCost = *max_element(a.begin(), a.end());

    if (isCoverable(maxCost + 1)) {
        println("infinite");
    } else {
        println(right);
    }

    return 0;
}

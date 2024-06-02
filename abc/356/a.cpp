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

    ll n, l, r;
    input(n, l, r);

    vector<ll> a(n);
    rep(i, n) {
        a[i] = i + 1;
    }
    // 順列をlrで3つに切る
    vector<ll> b1(a.begin() + l - 1, a.begin() + r), b2(a.begin(), a.begin() + l - 1), b3(a.begin() + r, a.end());
    vector<ll> ans;
    ans.insert(ans.end(), b2.begin(), b2.end());
    reverse(b1.begin(), b1.end());
    ans.insert(ans.end(), b1.begin(), b1.end());
    ans.insert(ans.end(), b3.begin(), b3.end());
    printv(ans);

    return 0;
}

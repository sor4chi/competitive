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
    vector<string> s(n);

    rep(i, n) input(s[i]);
    ll max_len = 0;
    rep(i, n) {
        max_len = max(max_len, (ll)s[i].size());
    }
    rep(i, n) {
        while (s[i].size() < max_len) {
            s[i].push_back('*');
        }
    }

    vector<string> ans(max_len);

    rep(i, max_len) {
        rep(j, n) {
            ans[i].push_back(s[n - j - 1][i]);
        }
    }

    rep(i, max_len) {
        while (ans[i].back() == '*') {
            ans[i].pop_back();
        }
    }

    rep(i, max_len) {
        println(ans[i]);
    }

    return 0;
}

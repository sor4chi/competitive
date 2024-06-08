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

    string s;
    input(s);

    ll upper_cnt = 0;
    ll lower_cnt = 0;
    rep(i, s.size()) {
        if (s[i] <= 'Z' && s[i] >= 'A') {
            upper_cnt++;
        } else {
            lower_cnt++;
        }
    }

    if (upper_cnt > lower_cnt) {
        rep(i, s.size()) {
            if (s[i] <= 'z' && s[i] >= 'a') {
                s[i] = s[i] - 'a' + 'A';
            }
        }
        println(s);
    } else {
        rep(i, s.size()) {
            if (s[i] <= 'Z' && s[i] >= 'A') {
                s[i] = s[i] - 'A' + 'a';
            }
        }
        println(s);
    }

    return 0;
}

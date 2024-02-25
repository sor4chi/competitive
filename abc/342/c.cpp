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

    ll n, q;
    string s;
    input(n, s, q);
    map<char, char> change_map;

    rep(i, 26) {
        char c = 'a' + i;
        change_map[c] = c;
    }

    rep(i, q) {
        char t, d;
        input(t, d);
        rep(j, 26) {
            char c = 'a' + j;
            if (change_map[c] == t) {
                change_map[c] = d;
            }
        }
    }

    rep(i, s.size()) {
        s[i] = change_map[s[i]];
    }

    println(s);

    return 0;
}

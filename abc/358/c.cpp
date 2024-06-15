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

    ll n, m;
    input(n, m);
    vector<string> s(n);
    rep(i, n) input(s[i]);

    ll ans = LLONG_MAX;
    rep(i, 1 << n) {
        vector<bool> is_filled(m, false);
        ll selected = 0;
        rep(j, n) {
            if (i >> j & 1) {
                selected++;
                rep(k, m) {
                    if (s[j][k] == 'o') {
                        is_filled[k] = true;
                    }
                }
            }
        }
        bool is_all_filled = true;
        rep(j, m) {
            if (!is_filled[j]) {
                is_all_filled = false;
                break;
            }
        }
        if (is_all_filled) {
            ans = min(ans, selected);
        }
    }

    println(ans);

    return 0;
}

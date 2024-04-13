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

    // S にちょうど i 回現れる文字はちょうど 0 種類またはちょうど 2 種類ある
    // という条件を満たす文字列 S が存在するかどうかを判定する

    map<char, int> m;  // 文字 -> 出現回数
    for (char c : s) {
        m[c]++;
    }

    // rev map
    map<int, int> rev;
    for (auto [k, v] : m) {
        rev[v]++;
    }

    bool ok = true;
    for (auto [k, v] : rev) {
        if (v != 2 && v != 0) {
            ok = false;
            break;
        }
    }

    yesno(ok);

    return 0;
}

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

    string s, t;
    input(s);
    input(t);
    bool is_end_X = false;
    rep(i, t.size()) {
        t[i] = tolower(t[i]);
    }
    is_end_X = t[2] == 'x';

    bool match_1 = false;
    bool match_2 = false;
    bool match_3 = false;

    rep(i, s.size()) {
        if (s[i] == t[0] && !match_1) {
            match_1 = true;
            continue;
        }
        if (s[i] == t[1] && match_1 && !match_2) {
            match_2 = true;
            continue;
        }
        if (s[i] == t[2] && match_2 && !match_3) {
            match_3 = true;
            continue;
        }
    }


    if (match_1 && match_2 && match_3) {
        println("Yes");
    } else {
        if (match_1 && match_2 && is_end_X) {
            println("Yes");
        } else {
            println("No");
        }
    }

    return 0;
}

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

    ll h, w, si, sj;
    input(h, w, si, sj);
    vector<string> s(h);
    rep(i, h) {
        input(s[i]);
    }
    string ops;
    input(ops);

    vector<pair<ll, ll>> dirs = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
    vector<string> dir_strs = {"U", "D", "L", "R"};

    auto is_valid = [&](ll i, ll j) {
        return 0 <= i && i < h && 0 <= j && j < w;
    };

    auto is_wall = [&](ll i, ll j) {
        return s[i][j] == '#';
    };

    ll i = si - 1, j = sj - 1;
    for (char op : ops) {
        ll dir = -1;
        rep(k, 4) {
            if (dir_strs[k][0] == op) {
                dir = k;
                break;
            }
        }
        ll ni = i + dirs[dir].first;
        ll nj = j + dirs[dir].second;
        if (!is_valid(ni, nj) || is_wall(ni, nj)) {
            continue;
        }
        i = ni;
        j = nj;
    }

    println(i + 1, j + 1);

    return 0;
}

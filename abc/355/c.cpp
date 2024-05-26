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
    int n, t;
    input(n, t);
    vector<int> A(t);
    rep(i, t) input(A[i]);

    unordered_map<int, pair<int, int>> position;
    int num = 1;
    rep(i, n) rep(j, n) position[num++] = {i, j};

    vector<int> row(n, 0), col(n, 0);
    int diag1 = 0, diag2 = 0;

    rep(t2, t) {
        int val = A[t2];
        if (position.find(val) == position.end()) continue;

        auto [r, c] = position[val];
        row[r]++;
        col[c]++;

        if (r == c) diag1++;
        if (r + c == n - 1) diag2++;

        if (row[r] == n || col[c] == n || diag1 == n || diag2 == n) {
            println(t2 + 1);
            return 0;
        }
    }

    println(-1);
    return 0;
}

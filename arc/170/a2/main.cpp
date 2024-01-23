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

bool is_valid_parentheses(const string& S) {
    ll cnt = 0;
    for (char c : S) {
        if (c == '(') {
            cnt++;
        } else {
            cnt--;
        }
        if (cnt < 0) {
            return false;
        }
    }
    return cnt == 0;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll N;
    string S, T, U;
    input(N, S, T);

    rep(i, N) {
        if (S[i] == 'A' && T[i] == 'B') {
            U += ')';
        } else if (S[i] == 'B' && T[i] == 'A') {
            U += '(';
        }
    }

    println(U);

    if (is_valid_parentheses(U)) {
        println(U.size() / 2);
        return 0;
    }

    if (U[0] == ')' || U[U.size() - 1] == '(') {
        println(-1);
        return 0;
    }

    return 0;
}

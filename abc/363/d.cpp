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

ll lpow(ll n, ll m) {
    ll ret = 1;
    for (ll i = 0; i < m; i++) {
        ret *= n;
    }
    return ret;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n;
    input(n);

    n--;

    if (n < 10) {
        println(n);
        return 0;
    }

    n -= 9;
    ll digit = 2;
    while (true) {
        // ある桁の桁数の回文数の個数を計算
        // 上の桁から決めていく
        // 桁が偶数の場合、(桁数 + 1) / 2桁、奇数のときは桁数-1と同じ
        ll halfCount = lpow(10, (digit + 1) / 2 - 1);
        if (n <= 9 * halfCount) {
            ll start = lpow(10, (digit - 1) / 2);
            ll offset = start + (n - 1);
            string str = to_string(offset);
            string rev = str;  // 逆順
            reverse(rev.begin(), rev.end());
            if (digit % 2 == 0) {
                println(str + rev);
                return 0;
            } else {
                println(str + rev.substr(1));
                return 0;
            }
        }
        n -= 9 * halfCount;
        digit++;
    }

    return 0;
}

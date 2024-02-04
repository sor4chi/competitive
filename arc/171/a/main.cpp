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

struct Case {
};

main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll T;
    input(T);
    rep(i, T) {
        ll N, A, B;  // Aがルーク,Bがポーン
        input(N, A, B);

        // まずルークがN個以上ならNoを出力
        if (A > N) {
            println("No");
            continue;
        }
        ll each = N - A;
        ll row = (N + 1) / 2;
        // println("row", row);
        if (N / 2 < A) {
            row -= A - N / 2;
        }
        ll max_pawn = row * each;
        // println("max_pawn", max_pawn);
        // println("each", each);
        // println("row", row);

        if (max_pawn >= B) {
            println("Yes");
        } else {
            println("No");
        }
    }

    return 0;
}

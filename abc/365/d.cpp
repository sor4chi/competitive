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

    ll n;
    input(n);
    string s;  // 青木くんの出した手
    input(s);  // RPSのどれか

    vector<vector<ll>> dp(n + 1, vector<ll>(3, 0));  // dp[i][j]: i回目までで最後高橋くんがjを出したときの最大の勝ち数

    // 高橋くんは全勝する

    rep(i, n) {
        rep(prev, 3) {
            rep(cur, 3) {
                if (cur != prev) {
                    bool is_loss = (s[i] == 'R' && cur == 2) || (s[i] == 'P' && cur == 0) || (s[i] == 'S' && cur == 1);
                    if (is_loss) {
                        continue;
                    }
                    ll win = (s[i] == 'R' && cur == 1) || (s[i] == 'P' && cur == 2) || (s[i] == 'S' && cur == 0);
                    dp[i + 1][cur] = max(dp[i + 1][cur], dp[i][prev] + win);
                }
            }
        }
    }

    // // DPテーブルを出力
    // eprintln("   R", "P", "S");
    // rep(i, n + 1) {
    //     cerr << (i == 0 ? ' ' : s[i - 1]) << ": ";
    //     eprintv(dp[i]);
    // }

    // n回目までで出した手のうち最大の勝ち数を出力
    println(*max_element(dp[n].begin(), dp[n].end()));

    return 0;
}

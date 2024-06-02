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

void printbitln(ll bit, ll n) {
    string s = "";
    for (ll i = 0; i < n; i++) {
        if (bit & (1 << i)) {
            s += "1";
        } else {
            s += "0";
        }
    }
    println(s);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, m, k;
    input(n, m, k);
    vector<pair<bool, vector<ll>>> tests;
    while (m--) {
        ll n2;
        input(n2);
        vector<ll> a(n2);
        rep(i, n2) input(a[i]);
        char okc;
        input(okc);
        tests.push_back({okc == 'o', a});
    }

    // // debug print a
    // for (auto [ok, a] : tests) {
    //     print(ok, " ");
    //     printv(a);
    // }

    // m桁のビット前探索
    ll cnt = 0;
    for (ll bit = 0; bit < (1 << n); bit++) {
        // 全てのテストケースについて
        bool all_ok = true;
        for (auto [ok, a] : tests) {
            if (ok) {
                // bit[a[i]]の中にk個以上1が含まれているか
                ll one_cnt = 0;
                for (ll i : a) {
                    if (bit & (1 << (i - 1))) {
                        one_cnt++;
                    }
                }
                if (one_cnt < k) {
                    all_ok = false;
                    break;
                }
            } else {
                // bit[a[i]]の中1の数がk個未満か
                ll one_cnt = 0;
                for (ll i : a) {
                    if (bit & (1 << (i - 1))) {
                        one_cnt++;
                    }
                }
                if (one_cnt >= k) {
                    all_ok = false;
                    break;
                }
            }
        }
        // bitをdebug print
        if (all_ok) {
            cnt++;
        }
    }

    println(cnt);

    return 0;
}

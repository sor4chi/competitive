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

    while (1) {
        ll n;
        input(n);
        if (n == 0) return 0;
        vector<ll> a(n), b(n);
        rep(i, n) input(a[i]);
        rep(i, n) input(b[i]);
        ll cnt = 0;
        ll prev_a_pos = 0;
        ll prev_b_pos = 0;
        int lead = 0;  // lead == 1 -> a lead == 2 -> b
        rep(i, n) {
            ll next_a_pos = prev_a_pos + a[i];
            ll next_b_pos = prev_b_pos + b[i];
            if (lead != 0) {
                // aについて
                if (lead == 1) {
                    if (next_b_pos > next_a_pos) {
                        cnt++;
                        lead = 2;
                    }
                } else if (lead == 2) {
                    if (next_a_pos > next_b_pos) {
                        cnt++;
                        lead = 1;
                    }
                }
            } else if (next_a_pos != next_b_pos) {
                if (next_b_pos > next_a_pos) {
                    lead = 2;
                } else {
                    lead = 1;
                }
            }
            prev_a_pos = next_a_pos;
            prev_b_pos = next_b_pos;
        }
        println(cnt);
    }

    return 0;
}

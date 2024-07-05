#include <bits/stdc++.h>

// #include <atcoder/all>

using namespace std;
// using namespace atcoder;
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
        ll n, m, p, q;
        input(n, m, p, q);
        vector<ll> a(m);
        rep(i, m) input(a[i]);
        if (n == 0 && m == 0 && p == 0 && q == 0) break;
        {
            ll cur = p;
            rep(i, m) {
                if (a[i] == cur) {
                    cur++;
                } else if (a[i] + 1 == cur) {
                    cur--;
                }
            }
            if (cur == q) {
                println("OK");
                goto end;
            }
        }

        rep(j, m + 1) {
            rep(k, n - 1) {
                vector<ll> b(a.begin(), a.begin() + j), c(a.begin() + j, a.end());
                b.push_back(k + 1);
                rep(i, c.size()) {
                    b.push_back(c[i]);
                }
                ll cur = p;
                rep(i, b.size()) {
                    if (b[i] == cur) {
                        cur++;
                    } else if (b[i] + 1 == cur) {
                        cur--;
                    }
                }
                if (cur == q) {
                    println(k + 1, j);
                    goto end;
                }
            }
        }

        println("NG");

    end:;
    }

    return 0;
}

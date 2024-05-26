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

struct Time {
    ll time;
    bool is_start;
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n;
    input(n);

    vector<pair<ll, ll>> lr(n);
    vector<Time> times;
    rep(i, n) {
        ll l, r;
        input(l, r);
        times.push_back({l, true});
        times.push_back({r + 1, false});
    }
    sort(times.begin(), times.end(), [](const Time& a, const Time& b) {
        if (a.time == b.time) return a.is_start < b.is_start;
        return a.time < b.time;
    });

    ll cur = 0;
    ll total = 0;
    for (const auto& time : times) {
        if (time.is_start) {
            total += cur;
            cur++;
        } else {
            cur--;
        }
    }

    println(total);

    return 0;
}

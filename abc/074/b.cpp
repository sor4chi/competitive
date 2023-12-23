#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
template <class T, class... Ts>
void print(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
    cout << '\n';
}
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void print() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)

int main() {
    ll n, k, total = 0;
    cin >> n >> k;

    for (ll i = 0; i < n; ++i) {
        ll a;
        cin >> a;
        if (a <= k / 2) {
            total += 2 * a;
        } else {
            total += 2 * (k - a);
        }
    }

    cout << total << endl;

    return 0;
}

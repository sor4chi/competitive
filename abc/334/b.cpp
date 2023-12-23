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

ll floor(ll x, ll m) {
    ll r = (x % m + m) % m;
    return (x - r) / m;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll from_point, span, left_point, right_point;
    input(from_point, span, left_point, right_point);

    left_point -= from_point;
    right_point -= from_point;

    print(floor(right_point, span) - floor(left_point - 1, span));

    return 0;
}

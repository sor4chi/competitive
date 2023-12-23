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

ll my_max_element(vector<ll> v) {
    ll max = -1000000000;
    for (ll i = 0; i < v.size(); ++i) {
        if (v[i] > max) max = v[i];
    }
    return max;
}

ll my_min_element(vector<ll> v) {
    ll min = 1000000000;
    for (ll i = 0; i < v.size(); ++i) {
        if (v[i] < min) min = v[i];
    }
    return min;
}

int main() {
    ll n, min = 1000000000;
    cin >> n;
    vector<ll> x(n);
    for (ll i = 0; i < n; ++i) cin >> x[i];
    ll max_x = my_max_element(x);
    ll min_x = my_min_element(x);

    for (ll i = min_x; i <= max_x; ++i) {
        ll sum = 0;
        for (ll j = 0; j < n; ++j) {
            sum += (x[j] - i) * (x[j] - i);
        }
        if (sum < min) min = sum;
    }

    cout << min << endl;

    return 0;
}

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
    ll n;
    cin >> n;
    float tax = 1.08;

    float raw = n / tax;
    ll raw_ceiled = ceil(raw);
    if (floor(raw_ceiled * tax) == n) {
        cout << raw_ceiled << endl;
    } else {
        cout << ":(" << endl;
    }

    return 0;
}

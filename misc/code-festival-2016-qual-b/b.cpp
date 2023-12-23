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
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, a, b, a_cnt = 0, b_cnt = 0;
    cin >> n >> a >> b;
    string s;
    cin >> s;

    for (ll i = 0; i < n; ++i) {
        if (s[i] == 'a') {
            if (a_cnt + b_cnt < a + b) {
                cout << "Yes" << endl;
                ++a_cnt;
            } else {
                cout << "No" << endl;
            }
        } else if (s[i] == 'b') {
            if (a_cnt + b_cnt < a + b && b_cnt < b) {
                cout << "Yes" << endl;
                ++b_cnt;
            } else {
                cout << "No" << endl;
            }
        } else {
            cout << "No" << endl;
        }
    }

    return 0;
}

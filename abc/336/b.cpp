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

    int N;
    input(N);

    string s;
    while (N > 0) {
        if (N % 2 == 0) {
            s += "0";
        } else {
            s += "1";
        }
        N /= 2;
    }

    int cnt = 0;
    for (auto c : s) {
        if (c == '0') {
            cnt++;
        } else {
            break;
        }
    }

    print(cnt);

    return 0;
}

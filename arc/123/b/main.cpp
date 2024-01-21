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
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void println() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll N;
    input(N);
    deque<ll> A(N), B(N), C(N);
    rep(i, N) input(A[i]);
    rep(i, N) input(B[i]);
    rep(i, N) input(C[i]);

    sort(A.begin(), A.end());
    sort(B.begin(), B.end());
    sort(C.begin(), C.end());

    ll a, b, c;
    ll ans = 0;
    while (!A.empty()) {
        a = A.front();
        A.pop_front();

        while (!B.empty() && B.front() <= a) B.pop_front();
        if (B.empty()) break;
        b = B.front();
        B.pop_front();

        while (!C.empty() && C.front() <= b) C.pop_front();
        if (C.empty()) break;
        c = C.front();
        C.pop_front();

        ans++;
    }

    println(ans);

    return 0;
}

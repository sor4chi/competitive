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

bool debug = false;

int cost(ll N, ll start, ll goal, ll cannot_pass_from) {
    if (goal > cannot_pass_from && cannot_pass_from >= start) {
        if (debug) println("DEBUG", "STARTとGOALの間にCANNOT_PASS_FROMがあるから、直接行く", start, goal, cannot_pass_from);
        return (N + start) - goal;
    }
    if (goal <= cannot_pass_from || cannot_pass_from < start) {
        if (debug) println("DEBUG", "STARTとGOALの間にCANNOT_PASS_FROMがないから、直接行く", start, goal, cannot_pass_from);
        return goal - start;
    }
    return min(goal - start, (N + start) - goal);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll N, M;
    input(N, M);

    vector<ll> X(M);
    rep(i, M) input(X[i]);

    const ll INF = 1LL << 60;
    ll min_total_cost = INF;
    rep(j, N) {
        if (debug) println("DEBUG", j + 1, "-", j + 2, "がCANNOT_PASS_FROMなとき");
        ll total_cost = 0;
        rep(i, M - 1) {
            ll start, goal;
            if (X[i] < X[i + 1]) {
                start = X[i];
                goal = X[i + 1];
            } else {
                start = X[i + 1];
                goal = X[i];
            }
            ll tmp = cost(N, start, goal, j + 1);
            total_cost += tmp;
            if (debug) println("DEBUG", "cost(", N, ", ", start, ", ", goal, ", ", j + 1, ") = ", tmp);
        }
        min_total_cost = min(min_total_cost, total_cost);
    }

    println(min_total_cost);

    return 0;
}


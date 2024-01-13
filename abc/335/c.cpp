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

    int n, q;
    input(n, q);
    vector<pair<ll, ll>> head_pos_history(q);
    rep(i, n) head_pos_history.push_back({n - i, 0});

    rep(i, q) {
        ll type;
        string s;
        input(type, s);
        if (type == 2) {
            int target = stoi(s) - 1;
            pair<ll, ll> target_pos = head_pos_history[head_pos_history.size() - 1 - target];
            print(target_pos.first, target_pos.second);
        } else {
            if (s == "L") {
                head_pos_history.push_back(
                    {head_pos_history.back().first - 1,
                     head_pos_history.back().second});
            } else if (s == "R") {
                head_pos_history.push_back(
                    {head_pos_history.back().first + 1,
                     head_pos_history.back().second});
            } else if (s == "U") {
                head_pos_history.push_back(
                    {head_pos_history.back().first,
                     head_pos_history.back().second + 1});
            } else if (s == "D") {
                head_pos_history.push_back(
                    {head_pos_history.back().first,
                     head_pos_history.back().second - 1});
            }
        }
    }
}

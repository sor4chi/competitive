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

struct OP {
    ll l, r, d;

    void print() {
        println(l, r, d);
    }
};

struct BeamNode {
    vector<OP> ops;
    vector<ll> v;
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, t;
    input(n, t);

    vector<vector<ll>> v(t, vector<ll>(n));
    rep(i, t) rep(j, n) input(v[i][j]);

    ll BEAM_SIZE = 2;
    vector<BeamNode> beam;
    beam.push_back({vector<OP>(), vector<ll>(n, 0)});

    ll SPAN = 5;
    rep(i, t) {
        vector<BeamNode> next_beam;
        for (auto node : beam) {
            vector<ll> evi;
            for (ll j = 0; j + SPAN < n; j += SPAN) {
                evi.push_back(j);
            }

            for (ll l = 0; l < evi.size(); l++) {
                for (ll r = l + 1; r < evi.size(); r++) {
                    for (ll d = -3; d <= 3; d++) {
                        vector<ll> next_v = node.v;
                        for (ll j = evi[l]; j < evi[r]; j++) {
                            next_v[j] += d;
                        }
                        vector<OP> next_ops = node.ops;
                        next_ops.push_back({evi[l], evi[r] - 1, d});
                        next_beam.push_back({next_ops, next_v});
                    }
                }
            }
        }

        cerr << "next_beam.size() = " << next_beam.size() << endl;

        vector<ll> goal_vt = v[i];
        auto eval = [&](BeamNode node) {
            ll score = 0;
            for (ll j = 0; j < n; j++) {
                score += abs(node.v[j] - goal_vt[j]);
            }
            return score;
        };

        sort(next_beam.begin(), next_beam.end(), [&](BeamNode a, BeamNode b) {
            return eval(a) < eval(b);
        });

        beam = vector<BeamNode>(next_beam.begin(), next_beam.begin() + BEAM_SIZE);
    }

    auto ops = beam[0].ops;

    for (auto op : ops) {
        op.print();
    }

    return 0;
}

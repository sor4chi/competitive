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
    int8_t l, r, d;

    void print() {
        // convert int8_t to int
        int l = this->l;
        int r = this->r;
        int d = this->d;
        println(l, r, d);
    }
};

struct BeamNode {
    vector<OP> ops;
    vector<int8_t> v;
};

int eval(vector<int8_t>& v, vector<int>& goal_vt) {
    int score = 0;
    for (int j = 0; j < v.size(); j++) {
        score += abs(v[j] - goal_vt[j]);
    }
    return score;
}

struct NextBeamNode {
    BeamNode node;
    int score;
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    int n, t;
    input(n, t);

    vector<vector<int>> v(t, vector<int>(n));
    rep(i, t) rep(j, n) input(v[i][j]);

    int BEAM_SIZE = 6;
    vector<BeamNode> beam;
    beam.push_back({vector<OP>(), vector<int8_t>(n, 0)});

    int8_t SPAN = 4;
    vector<int8_t> evi;
    for (int8_t j = 0; j + SPAN < n; j += SPAN) {
        evi.push_back(j);
    }
    for (int i = 0; i < t; i++) {
        vector<NextBeamNode> next_beam;
        for (auto node : beam) {
            for (int8_t l = 0; l < evi.size(); l++) {
                for (int8_t r = l + 1; r < evi.size(); r++) {
                    for (int8_t d = -3; d <= 3; d++) {
                        vector<int8_t> next_v = node.v;
                        for (int8_t j = evi[l]; j < evi[r]; j++) {
                            next_v[j] += d;
                        }
                        vector<OP> next_ops = node.ops;
                        next_ops.push_back({evi[l], (int8_t)(evi[r] - 1), d});
                        next_beam.push_back({next_ops, next_v, eval(next_v, v[i])});
                    }
                }
            }
        }

        cerr << "next_beam.size() = " << next_beam.size() << endl;

        vector<int> goal_vt = v[i];

        sort(next_beam.begin(), next_beam.end(), [&](NextBeamNode a, NextBeamNode b) {
            return a.score < b.score;
        });

        beam.clear();

        for (int j = 0; j < min(BEAM_SIZE, (int)next_beam.size()); j++) {
            beam.push_back(next_beam[j].node);
        }
    }

    auto ops = beam[0].ops;

    for (auto op : ops) {
        op.print();
    }

    return 0;
}

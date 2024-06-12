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

int counter = 0;
int gen_id() {
    return counter++;
}

struct BeamNode {
    int parent_id;
    int id;
    OP op;
    vector<int8_t> v;
    int score;

    bool operator<(const BeamNode& other) const {
        return score < other.score;
    }
};

int eval(vector<int8_t>& v, vector<int>& goal_vt) {
    int score = 0;
    for (int j = 0; j < v.size(); j++) {
        score += abs(v[j] - goal_vt[j]);
    }
    return score;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    int n, t;
    input(n, t);

    vector<vector<int>> v(t, vector<int>(n));
    rep(i, t) rep(j, n) input(v[i][j]);

    int BEAM_SIZE = 60;
    priority_queue<BeamNode> beam;
    map<int, BeamNode> beam_history;
    beam.push({-1, gen_id(), {}, vector<int8_t>(n, 0)});
    beam_history[0] = beam.top();

    int8_t SPAN = 4;
    vector<int8_t> evi;
    for (int8_t j = 0; j + SPAN < n; j += SPAN) {
        evi.push_back(j);
    }
    if (evi.back() != n - 1) {
        evi.push_back(n - 1);
    }
    for (int i = 0; i < t; i++) {
        priority_queue<BeamNode> next_beam;

        while (!beam.empty()) {
            auto node = beam.top();
            beam.pop();
            for (int8_t l = 0; l < evi.size(); l++) {
                for (int8_t r = l + 1; r < evi.size(); r++) {
                    for (int8_t d = -3; d <= 3; d++) {
                        vector<int8_t> next_v = node.v;
                        for (int8_t j = evi[l]; j < evi[r]; j++) {
                            next_v[j] += d;
                        }
                        next_beam.push({node.id, -1, {evi[l], (int8_t)(evi[r] - 1), d}, next_v, eval(next_v, v[i])});
                        if (next_beam.size() > BEAM_SIZE) {
                            next_beam.pop();
                        }
                    }
                }
            }
        }

        cerr << "next_beam.size() = " << next_beam.size() << endl;

        beam = priority_queue<BeamNode>();
        while (!next_beam.empty()) {
            auto node = next_beam.top();
            node.id = gen_id();
            beam.push(node);
            next_beam.pop();

            beam_history[node.id] = node;
        }
    }

    vector<OP> ops;
    BeamNode best_node;
    while (!beam.empty()) {
        best_node = beam.top();
        beam.pop();
    }

    while (best_node.parent_id != -1) {
        ops.push_back(beam_history[best_node.id].op);
        best_node = beam_history[best_node.parent_id];
    }

    reverse(ops.begin(), ops.end());

    for (auto op : ops) {
        op.print();
    }

    return 0;
}

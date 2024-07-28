
#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
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
void println() { cout << '\n'; }
template <class T, class... Ts>
void eprintln(const T& a, const Ts&... b) {
    cerr << a;
    (void)(cerr << ... << (cerr << ' ', b));
    cerr << '\n';
}
template <class T>
void eprintv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cerr << x << sep);
    }
    cerr << end;
}
void eprintln() { cerr << '\n'; }
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

using mint = modint998244353;

struct Input {
    ll n, m, k;
    vector<vector<ll>> a;
    vector<vector<vector<ll>>> stamps;
};

struct Step {
    ll stamp_id, x, y;
};

unsigned long rng() {
    static unsigned long x = 88172645463325252UL;
    x ^= x << 7;
    x ^= x >> 9;
    return x;
}

double rng_double() {
    return rng() / (ULONG_MAX + 1.0);
}

template <typename container_type>
vector<vector<typename container_type::value_type>>
combWithReplace(container_type const& choices, size_t n) {
    using value_type = typename container_type::value_type;
    using selected_t = vector<value_type>;
    using itor_t = typename container_type::const_iterator;
    struct impl {                        // lambda で再帰は面倒なので クラスにする
        vector<vector<value_type>>& r_;  // コピーを避けるために参照を持つ
        impl(vector<vector<value_type>>& r) : r_(r) {}
        void append(selected_t& s, itor_t b, itor_t e, size_t n) {
            if (n == 0) {
                r_.push_back(s);
            } else {
                for (auto it = b; it != e; ++it) {
                    s.push_back(*it);
                    append(s, it, e, n - 1);
                    s.pop_back();
                }
            }
        };
    };
    vector<vector<value_type>> r;
    impl o{r};
    selected_t e;
    e.reserve(n);
    o.append(e, cbegin(choices), cend(choices), n);
    return r;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    Input in;
    input(in.n, in.m, in.k);
    in.a.resize(in.n, vector<ll>(in.n));
    rep(i, in.n) rep(j, in.n) input(in.a[i][j]);
    in.stamps.resize(in.m);
    rep(i, in.m) {
        vector<vector<ll>> s(3, vector<ll>(3));
        rep(i, 3) rep(j, 3) input(s[i][j]);
        in.stamps[i] = s;
    }

    vector<Step> final_ans;
    vector<vector<mint>> board = vector<vector<mint>>(9, vector<mint>(9));
    rep(i, in.n) rep(j, in.n) {
        board[i][j] = in.a[i][j];
    }

    vector<ll> stamps_permutation(in.m);
    iota(stamps_permutation.begin(), stamps_permutation.end(), 0);

    rep(i, 6) {
        rep(j, 6) {
            ll max_score = board[i][j].val();
            ll max_stid = -1;
            rep(stid, in.m) {
                mint new_val = board[i][j] + in.stamps[stid][0][0];
                ll score = new_val.val();
                if (score > max_score) {
                    max_score = score;
                    max_stid = stid;
                }
            }
            if (max_stid != -1) {
                final_ans.push_back({max_stid, i, j});
                rep(i_sub, 3) rep(j_sub, 3) {
                    board[i + i_sub][j + j_sub] += in.stamps[max_stid][i_sub][j_sub];
                }
            }
        }
    }

    rep(i, 6) {
        ll max_score = board[i][6].val() + board[i][7].val() + board[i][8].val();
        vector<ll> max_stid_ops;
        vector<vector<ll>> max_stid_candidates;
        rep1(i, 3) {
            for (auto stid_ops : combWithReplace(stamps_permutation, i)) {
                max_stid_candidates.push_back(stid_ops);
            }
        }
        for (auto stid_ops : max_stid_candidates) {
            vector<mint> board_row = {board[i][6], board[i][7], board[i][8]};
            for (auto stid : stid_ops) {
                rep(j, 3) {
                    board_row[j] += in.stamps[stid][0][j];
                }
            }
            ll score = board_row[0].val() + board_row[1].val() + board_row[2].val();
            if (score > max_score) {
                max_score = score;
                max_stid_ops = stid_ops;
            }
        }
        for (auto stid : max_stid_ops) {
            final_ans.push_back({stid, i, 6});
            rep(i_sub, 3) rep(j_sub, 3) {
                board[i + i_sub][6 + j_sub] += in.stamps[stid][i_sub][j_sub];
            }
        }
    }

    rep(j, 6) {
        ll max_score = board[6][j].val() + board[7][j].val() + board[8][j].val();
        vector<ll> max_stid_ops;
        vector<vector<ll>> max_stid_candidates;
        rep1(i, 3) {
            for (auto stid_ops : combWithReplace(stamps_permutation, i)) {
                max_stid_candidates.push_back(stid_ops);
            }
        }
        for (auto stid_ops : max_stid_candidates) {
            vector<mint> board_col = {board[6][j], board[7][j], board[8][j]};
            for (auto stid : stid_ops) {
                rep(i, 3) {
                    board_col[i] += in.stamps[stid][i][0];
                }
            }
            ll score = board_col[0].val() + board_col[1].val() + board_col[2].val();
            if (score > max_score) {
                max_score = score;
                max_stid_ops = stid_ops;
            }
        }
        for (auto stid : max_stid_ops) {
            final_ans.push_back({stid, 6, j});
            rep(i_sub, 3) rep(j_sub, 3) {
                board[6 + i_sub][j + j_sub] += in.stamps[stid][i_sub][j_sub];
            }
        }
    }

    ll max_score = 0;
    rep(i, 3) rep(j, 3) {
        max_score += board[6 + i][6 + j].val();
    }
    vector<ll> max_stid_ops;
    vector<vector<ll>> max_stid_candidates;
    rep1(i, 8) {
        for (auto stid_ops : combWithReplace(stamps_permutation, i)) {
            max_stid_candidates.push_back(stid_ops);
        }
    }
    for (auto stid_ops : max_stid_candidates) {
        vector<vector<mint>> board_sub = {{board[6][6], board[6][7], board[6][8]},
                                          {board[7][6], board[7][7], board[7][8]},
                                          {board[8][6], board[8][7], board[8][8]}};
        for (auto stid : stid_ops) {
            rep(i, 3) rep(j, 3) {
                board_sub[i][j] += in.stamps[stid][i][j];
            }
        }
        ll score = 0;
        rep(i, 3) rep(j, 3) {
            score += board_sub[i][j].val();
        }
        if (score > max_score) {
            max_score = score;
            max_stid_ops = stid_ops;
        }
    }

    for (auto stid : max_stid_ops) {
        final_ans.push_back({stid, 6, 6});
        rep(i_sub, 3) rep(j_sub, 3) {
            board[6 + i_sub][6 + j_sub] += in.stamps[stid][i_sub][j_sub];
        }
    }

    println(final_ans.size());
    for (auto s : final_ans) {
        println(s.stamp_id, s.x, s.y);
    }

    return 0;
}

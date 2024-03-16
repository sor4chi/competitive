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

struct Tile {
    ll h, w;
};

bool fillable(vector<vector<bool>>& filled, const Tile& tile, ll x, ll y) {
    if (x + tile.w > filled[0].size() || y + tile.h > filled.size()) {
        return false;
    }
    for (ll i = y; i < y + tile.h; i++) {
        for (ll j = x; j < x + tile.w; j++) {
            if (filled[i][j]) {
                return false;
            }
        }
    }
    return true;
}

void fill(vector<vector<bool>>& filled, const Tile& tile, ll x, ll y) {
    for (ll i = y; i < y + tile.h; i++) {
        for (ll j = x; j < x + tile.w; j++) {
            filled[i][j] = true;
        }
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, h, w;
    input(n, h, w);
    vector<Tile> tiles(n);
    rep(i, n) input(tiles[i].h, tiles[i].w);

    random_device seed_gen;
    mt19937 engine(seed_gen());

    chrono::system_clock::time_point start = chrono::system_clock::now();
    while (chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count() < 1900) {
        shuffle(tiles.begin(), tiles.end(), engine);
        vector<vector<bool>> filled(h, vector<bool>(w, false));
        ll filled_count = 0;
        for (auto tile : tiles) {
            rep(j, h) {
                rep(k, w) {
                    bool first_swap = rand() % 2;
                    if (first_swap) {
                        swap(tile.h, tile.w);
                    }
                    if (fillable(filled, tile, k, j)) {
                        fill(filled, tile, k, j);
                        filled_count += tile.h * tile.w;
                        goto filled;
                    }
                    swap(tile.h, tile.w);
                    if (fillable(filled, tile, k, j)) {
                        fill(filled, tile, k, j);
                        filled_count += tile.h * tile.w;
                        goto filled;
                    }
                    swap(tile.h, tile.w);
                }
            }
            break;
        filled:
            continue;
        }

        if (filled_count == h * w) {
            println("Yes");
            return 0;
        }
    }

    println("No");

    return 0;
}

#include <bits/stdc++.h>
using namespace std;

struct Input {
    int N;                            // The grid size. 8 <= N <= 30.
    int C;                            // The number of colours. 2 <= C <= 6.
    vector<vector<int>> start_grid;   // The initial grid. Each element is a colour. size: N x N. -1 is wall.
    vector<vector<int>> target_grid;  // The target grid. Each element is a colour. size: N x N. -1 is wall.

    void scan() {
        cin >> N >> C;
        start_grid = vector<vector<int>>(N, vector<int>(N));
        target_grid = vector<vector<int>>(N, vector<int>(N));
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                cin >> start_grid[i][j];
            }
        }
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                cin >> target_grid[i][j];
            }
        }
    }

    void debug_print() {
        cerr << "N: " << N << endl;
        cerr << "C: " << C << endl;
        cerr << "Start grid:" << endl;
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                cerr << setw(2) << start_grid[i][j] << " ";
            }
            cerr << endl;
        }
        cerr << "Target grid:" << endl;
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                cerr << setw(2) << target_grid[i][j] << " ";
            }
            cerr << endl;
        }
    }
};

const int TIME_LIMIT = 9800;  // 10s (- alpha)

namespace game {
const int dx[] = {0, 1, 0, -1, 1, 1, -1, -1};
const int dy[] = {1, 0, -1, 0, 1, -1, 1, -1};
bool out_of_range(int x, int y, int N) {
    return x < 0 || x >= N || y < 0 || y >= N;
}

struct Operation {
    int x, y;
    int color;

    // for set
    bool operator<(const Operation& other) const {
        if (x != other.x) return x < other.x;
        if (y != other.y) return y < other.y;
        return color < other.color;
    }
};

struct Reversi {
    int N;
    vector<vector<int>> grid;

    Reversi(Input in) {
        N = in.N;
        grid = in.start_grid;
    }

    set<Operation> get_valid_moves() {
        set<Operation> moves;
        for (int x = 0; x < N; x++) {
            for (int y = 0; y < N; y++) {
                if (grid[x][y] <= 0) continue;  // ignore wall, empty
                int color = grid[x][y];
                for (int i = 0; i < 8; i++) {
                    int cur_x = x + dx[i];
                    int cur_y = y + dy[i];
                    int d = 1;
                    while (true) {
                        if (out_of_range(cur_x, cur_y, N)) break;  // out of range
                        if (grid[cur_x][cur_y] == color) break;    // same color
                        if (grid[cur_x][cur_y] == -1) break;       // wall
                        if (grid[cur_x][cur_y] == 0) {
                            if (d > 1) {
                                moves.insert({cur_x, cur_y, color});
                            }
                            break;
                        }
                        cur_x += dx[i];
                        cur_y += dy[i];
                        d++;
                    }
                }
            }
        }

        return moves;
    }

    int apply_move(Operation move, Input& in) {
        int valid_color_diff = 0;
        grid[move.x][move.y] = move.color;
        int prev_color = in.start_grid[move.x][move.y];
        int target_color = in.target_grid[move.x][move.y];
        // if prev_color is not target_color and new color is target_color, increment valid_color_diff
        if (prev_color != target_color && move.color == target_color) {
            valid_color_diff++;
        }
        // if prev_color is target_color and new color is not target_color, decrement valid_color_diff
        if (prev_color == target_color && move.color != target_color) {
            valid_color_diff--;
        }
        for (int i = 0; i < 8; i++) {
            int cur_x = move.x + dx[i];
            int cur_y = move.y + dy[i];
            bool valid = false;
            while (true) {
                if (out_of_range(cur_x, cur_y, N)) break;  // out of range
                if (grid[cur_x][cur_y] == -1) break;       // wall
                if (grid[cur_x][cur_y] == 0) break;        // empty
                if (grid[cur_x][cur_y] == move.color) {
                    valid = true;
                    break;
                }
                cur_x += dx[i];
                cur_y += dy[i];
            }
            if (valid) {
                cur_x = move.x + dx[i];
                cur_y = move.y + dy[i];
                while (true) {
                    if (grid[cur_x][cur_y] == move.color) break;
                    int prev_color = grid[cur_x][cur_y];
                    int target_color = in.target_grid[cur_x][cur_y];
                    grid[cur_x][cur_y] = move.color;
                    // if prev_color is not target_color and new color is target_color, increment valid_color_diff
                    if (prev_color != target_color && move.color == target_color) {
                        valid_color_diff++;
                    }
                    // if prev_color is target_color and new color is not target_color, decrement valid_color_diff
                    if (prev_color == target_color && move.color != target_color) {
                        valid_color_diff--;
                    }
                    cur_x += dx[i];
                    cur_y += dy[i];
                }
            }
        }
        return valid_color_diff;
    }

    void debug_print() {
        cerr << "Grid:" << endl;
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                cerr << setw(2) << grid[i][j] << " ";
            }
            cerr << endl;
        }
    }
};
}  // namespace game

namespace utility {
struct TimeKeeper {
    chrono::system_clock::time_point start;
    TimeKeeper() {
        start = chrono::system_clock::now();
    }
    int elapsed() {
        return chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start).count();
    }
    int is_timeout(int tl) {
        return elapsed() > tl;
    }
};

}  // namespace utility

namespace solver {
int eval(const vector<vector<int>>& grid, const vector<vector<int>>& start_grid, const vector<vector<int>>& target_grid) {
    int draw_cnt = 0;
    int valid_cnt = 0;
    int N = grid.size();
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < N; j++) {
            if (grid[i][j] > 0) {
                if (start_grid[i][j] == 0) {
                    draw_cnt++;
                }
                if (grid[i][j] == target_grid[i][j]) {
                    valid_cnt++;
                }
            }
        }
    }
    return draw_cnt + valid_cnt * valid_cnt;
}

pair<int, vector<game::Operation>> solve_randomly(Input in, game::Reversi& game) {
    vector<game::Operation> moves;
    while (true) {
        auto valid_moves = game.get_valid_moves();
        if (valid_moves.empty()) break;

        vector<game::Operation> valid_moves_vec(valid_moves.begin(), valid_moves.end());
        game::Operation move = valid_moves_vec[rand() % valid_moves_vec.size()];
        game.apply_move(move, in);
        moves.push_back(move);

        // game.debug_print();
    }

    int score = eval(game.grid, in.start_grid, in.target_grid);
    cerr << "Score: " << score << endl;

    return {score, moves};
}

pair<int, vector<game::Operation>> solve_dfs(Input in, game::Reversi& game, utility::TimeKeeper& tk, int tl, int each_tl, utility::TimeKeeper& trial_tk) {
    int best_score = eval(game.grid, in.start_grid, in.target_grid);
    vector<game::Operation> best_moves;

    stack<pair<game::Reversi, vector<game::Operation>>> st;
    st.push({game, {}});

    random_device seed_gen;
    mt19937 engine(seed_gen());

    double start_temp = 1000.0;
    double end_temp = 0.1;

    while (!st.empty()) {
        if (tk.is_timeout(tl)) break;
        if (trial_tk.is_timeout(each_tl)) break;
        auto [cur_game, cur_moves] = st.top();
        st.pop();

        auto valid_moves = cur_game.get_valid_moves();

        int score = eval(cur_game.grid, in.start_grid, in.target_grid);

        int diff = score - best_score;
        double temp = start_temp + (end_temp - start_temp) * trial_tk.elapsed() / each_tl;
        double prob = exp(diff / temp);
        if (diff > 0 || prob > uniform_real_distribution<double>(0.0, 1.0)(engine)) {
            if (score > best_score) {
                best_score = score;
                best_moves = cur_moves;
            }
        } else {
            continue;
        }

        vector<game::Operation> valid_moves_vec(valid_moves.begin(), valid_moves.end());

        shuffle(valid_moves_vec.begin(), valid_moves_vec.end(), engine);

        for (auto move : valid_moves_vec) {
            game::Reversi next_game = cur_game;
            next_game.apply_move(move, in);
            vector<game::Operation> next_moves = cur_moves;
            next_moves.push_back(move);
            st.push({next_game, next_moves});
        }
    }

    cerr << "Best score: " << best_score << endl;

    return {best_score, best_moves};
}

pair<int, vector<game::Operation>> solve_beam_search(Input in, game::Reversi& game, utility::TimeKeeper& tk) {
    struct Beam {
        int score;
        int filled_cnt;
        int valid_color_cnt;
        vector<game::Operation> moves;
        game::Reversi game;
    };
    vector<Beam> beams;
    int valid_color_cnt = 0;
    for (int i = 0; i < in.N; i++) {
        for (int j = 0; j < in.N; j++) {
            if (game.grid[i][j] == in.target_grid[i][j] && in.target_grid[i][j] > 0) {
                valid_color_cnt++;
            }
        }
    }
    beams.push_back({valid_color_cnt * valid_color_cnt, 0, valid_color_cnt, {}, game});
    vector<game::Operation> best_moves;
    int best_score = 0;
    int wall_cnt = 0;
    for (int i = 0; i < in.N; i++) {
        for (int j = 0; j < in.N; j++) {
            if (game.grid[i][j] == -1) {
                wall_cnt++;
            }
        }
    }
    int fillable_cnt = in.N * in.N - wall_cnt;
    int beam_width = 7000000 / (fillable_cnt * fillable_cnt);
    cerr << "Beam width: " << beam_width << endl;

    random_device seed_gen;
    mt19937 engine(seed_gen());

    while (!beams.empty()) {
        if (tk.elapsed() > TIME_LIMIT * 0.9) {
            beam_width = 1;
        }
        if (tk.elapsed() > TIME_LIMIT) break;
        vector<Beam> next_beams;
        for (auto& beam : beams) {
            auto valid_moves = beam.game.get_valid_moves();

            for (auto move : valid_moves) {
                game::Reversi next_game = beam.game;
                auto valid_color_diff = next_game.apply_move(move, in);
                int next_filled_cnt = beam.filled_cnt + 1;
                int next_valid_color_cnt = beam.valid_color_cnt + valid_color_diff;
                vector<game::Operation> next_moves = beam.moves;
                next_moves.push_back(move);
                int score = next_filled_cnt + next_valid_color_cnt * next_valid_color_cnt + engine() % 100;
                if (score > best_score) {
                    best_score = score;
                    best_moves = next_moves;
                }
                next_beams.push_back({score, next_filled_cnt, next_valid_color_cnt, next_moves, next_game});
            }
        }

        if (next_beams.empty()) break;

        sort(next_beams.begin(), next_beams.end(), [&](const Beam& a, const Beam& b) {
            return a.score > b.score;
        });

        // log top score
        cerr << "Top score: " << next_beams[0].score << ", progress: " << next_beams[0].moves.size() << " / " << fillable_cnt << ", beam width: " << beam_width << endl;

        beams.clear();
        for (int i = 0; i < min(beam_width, (int)next_beams.size()); i++) {
            beams.push_back(next_beams[i]);
        }
    }

    return {best_score, best_moves};
}
}  // namespace solver

int main() {
    utility::TimeKeeper tk;
    Input in;
    in.scan();
    // in.debug_print();

    game::Reversi game(in);

    auto [score, moves] = solver::solve_beam_search(in, game, tk);

    cout << moves.size() << endl;
    for (auto move : moves) {
        cout << move.x << " " << move.y << " " << move.color << endl;
    }

    return 0;
}

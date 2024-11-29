#include <algorithm>
#include <chrono>
#include <climits>
#include <iostream>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <vector>

using namespace std;

namespace io {
struct Input {
    int N;
    int C;
    int K;
    vector<vector<int>> START;
    vector<vector<int>> TARGET;

    Input() {
        cin >> N >> C >> K;
        cerr << N << " " << C << " " << K << endl;
        START.resize(N, vector<int>(N));
        TARGET.resize(N, vector<int>(N));
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                cin >> START[i][j];
                cerr << START[i][j] << " ";
            }
            cerr << endl;
        }
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                cin >> TARGET[i][j];
                cerr << TARGET[i][j] << " ";
            }
            cerr << endl;
        }
    }
};

enum OperationType {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    PLACE
};

char operationToChar(OperationType operation) {
    switch (operation) {
        case UP:
            return 'U';
        case DOWN:
            return 'D';
        case LEFT:
            return 'L';
        case RIGHT:
            return 'R';
        case PLACE:
            return 'B';
    }
}

struct Operation {
    OperationType type;
    int r;
    int c;

    Operation(int r, int c, OperationType type) : r(r), c(c), type(type) {}
};

struct Output {
    int M;
    vector<Operation> operations;

    Output(int M, vector<Operation> operations) : M(M), operations(operations) {}

    void print() {
        cout << M << endl;
        for (Operation op : operations) {
            cout << op.r << " " << op.c << " " << operationToChar(op.type) << endl;
        }
    }
};
}  // namespace io

namespace solver {
using namespace io;
pair<int, int> getDirection(OperationType direction) {
    switch (direction) {
        case UP:
            return {-1, 0};
        case DOWN:
            return {1, 0};
        case LEFT:
            return {0, -1};
        case RIGHT:
            return {0, 1};
        case PLACE:
            return {0, 0};
    }
}

OperationType getOppositeDirection(OperationType direction) {
    switch (direction) {
        case UP:
            return OperationType::DOWN;
        case DOWN:
            return OperationType::UP;
        case LEFT:
            return OperationType::RIGHT;
        case RIGHT:
            return OperationType::LEFT;
        case PLACE:
            return OperationType::PLACE;
    }
}

const vector<OperationType> DIRECTIONS = {OperationType::UP, OperationType::DOWN, OperationType::LEFT, OperationType::RIGHT};

struct Point {
    int r;
    int c;

    Point() : r(-1), c(-1) {}
    Point(int r, int c) : r(r), c(c) {}

    bool operator==(const Point &other) const {
        return r == other.r && c == other.c;
    }

    bool operator!=(const Point &other) const {
        return !(*this == other);
    }

    bool operator<(const Point &other) const {
        return r < other.r || (r == other.r && c < other.c);
    }

    bool valid(int N) {
        return r >= 0 && r < N && c >= 0 && c < N;
    }
};

struct Solver {
    Input input;

    Solver(Input input) : input(input) {}

    Output greedy() {
        chrono::steady_clock::time_point begin = chrono::steady_clock::now();
        int tl = 9000;

        int iter = 1;
        int WALL = -1;
        int best_score = INT_MAX;
        vector<Operation> best_operations;
        random_device seed_gen;
        mt19937 engine(seed_gen());

        vector<int> perm(input.N * input.N);
        iota(perm.begin(), perm.end(), 0);

        vector<Point> warning_points;
        for (int i = 0; i < input.N; i++) {
            for (int j = 0; j < input.N; j++) {
                if (input.TARGET[i][j] != 0 || input.START[i][j] != 0) {
                    continue;
                }
                int surrounded = 0;
                for (OperationType direction : DIRECTIONS) {
                    auto [dr, dc] = getDirection(direction);
                    int r = i + dr;
                    int c = j + dc;
                    if (r >= 0 && r < input.N && c >= 0 && c < input.N) {
                        if (input.TARGET[r][c] > 0) {
                            surrounded++;
                        }
                    }
                }
                if (surrounded >= 2) {
                    warning_points.push_back(Point(i, j));
                }
            }
        }

        while (chrono::steady_clock::now() - begin < chrono::milliseconds(tl)) {
            vector<Operation> operations;
            vector<vector<int>> board = input.START;

            int moves = 0;
            int places = 0;

            int fill_count = engine() % (warning_points.size() / 4 + 2);
            shuffle(warning_points.begin(), warning_points.end(), engine);
            for (int i = 0; i < min(fill_count, (int)warning_points.size()); i++) {
                Point p = warning_points[i];
                board[p.r][p.c] = WALL;
                places++;
                operations.push_back(Operation(p.r, p.c, OperationType::PLACE));
            }

            while (1) {
                vector<Operation> current_operations;

                bool updates = false;
                shuffle(perm.begin(), perm.end(), engine);
                for (int t = 0; t < input.N * input.N; t++) {
                    int i = perm[t] / input.N;
                    int j = perm[t] % input.N;
                    if (board[i][j] > 0 && board[i][j] != input.TARGET[i][j]) {
                        int target_c = board[i][j];
                        queue<tuple<Point, OperationType, int>> q;
                        q.push({Point(i, j), PLACE, 0});
                        map<Point, Point> prev;
                        vector<Point> goals;

                        while (!q.empty()) {
                            auto [pos, prev_dir, depth] = q.front();
                            auto [r, c] = pos;
                            q.pop();

                            if (input.TARGET[r][c] == target_c) {
                                goals.push_back(Point(r, c));
                                break;
                            }

                            vector<tuple<Point, OperationType, int>> next_points;
                            for (OperationType direction : DIRECTIONS) {
                                if (getOppositeDirection(direction) == prev_dir) {
                                    continue;
                                }
                                auto [dr, dc] = getDirection(direction);
                                int rc = r;
                                int cc = c;
                                while (rc + dr >= 0 && rc + dr < input.N && cc + dc >= 0 && cc + dc < input.N && ((rc + dr == i && cc + dc == j) || board[rc + dr][cc + dc] == 0)) {
                                    rc += dr;
                                    cc += dc;
                                }
                                if (prev.count(Point(rc, cc)) == 0 && depth < 5) {
                                    next_points.push_back({Point(rc, cc), direction, depth + 1});
                                    prev[Point(rc, cc)] = Point(r, c);
                                }
                            }

                            shuffle(next_points.begin(), next_points.end(), engine);

                            for (auto [next_point, direction, next_depth] : next_points) {
                                q.push({next_point, direction, next_depth});
                            }
                        }

                        if (goals.empty()) {
                            continue;
                        }

                        Point goal = goals[engine() % goals.size()];
                        Point current = goal;
                        vector<Operation> path;
                        while (current != Point(i, j)) {
                            auto [r, c] = current;
                            auto [pr, pc] = prev[current];
                            if (r > pr) {
                                path.push_back(Operation(pr, pc, OperationType::DOWN));
                            } else if (r < pr) {
                                path.push_back(Operation(pr, pc, OperationType::UP));
                            } else if (c > pc) {
                                path.push_back(Operation(pr, pc, OperationType::RIGHT));
                            } else if (c < pc) {
                                path.push_back(Operation(pr, pc, OperationType::LEFT));
                            }
                            current = {pr, pc};
                            moves++;
                        }

                        for (int i = path.size() - 1; i >= 0; i--) {
                            current_operations.push_back(path[i]);
                        }

                        board[goal.r][goal.c] = target_c;
                        board[i][j] = 0;
                        updates = true;
                    }
                }

                if (!updates) {
                    break;
                } else {
                    for (Operation op : current_operations) {
                        operations.push_back(op);
                    }
                }
            }

            for (int i = 0; i < input.N; i++) {
                for (int j = 0; j < input.N; j++) {
                    if (board[i][j] > 0 && input.TARGET[i][j] == 0) {
                        queue<tuple<Point, OperationType, int>> q;
                        q.push({Point(i, j), PLACE, 0});
                        map<Point, Point> prev;
                        Point goal = Point(-1, -1);

                        while (!q.empty()) {
                            auto [pos, prev_dir, depth] = q.front();
                            auto [r, c] = pos;
                            q.pop();

                            if (input.TARGET[r][c] > 0) {
                                goal = Point(r, c);
                                break;
                            }

                            vector<tuple<Point, OperationType, int>> nexts;
                            for (OperationType direction : DIRECTIONS) {
                                if (getOppositeDirection(direction) == prev_dir) {
                                    continue;
                                }
                                auto [dr, dc] = getDirection(direction);
                                int rc = r;
                                int cc = c;
                                while (rc + dr >= 0 && rc + dr < input.N && cc + dc >= 0 && cc + dc < input.N && ((rc + dr == i && cc + dc == j) || board[rc + dr][cc + dc] == 0)) {
                                    rc += dr;
                                    cc += dc;
                                }
                                if (prev.count(Point(rc, cc)) == 0 && depth < 5) {
                                    nexts.push_back({Point(rc, cc), direction, depth + 1});
                                    prev[Point(rc, cc)] = Point(r, c);
                                }
                            }

                            shuffle(nexts.begin(), nexts.end(), engine);

                            for (auto [next_pos, next_dir, next_depth] : nexts) {
                                q.push({next_pos, next_dir, next_depth});
                            }
                        }

                        if (goal == Point(-1, -1)) {
                            continue;
                        }

                        Point current = goal;
                        vector<Operation> path;
                        while (current != Point(i, j)) {
                            auto [r, c] = current;
                            auto [pr, pc] = prev[current];
                            if (r > pr) {
                                path.push_back(Operation(pr, pc, OperationType::DOWN));
                            } else if (r < pr) {
                                path.push_back(Operation(pr, pc, OperationType::UP));
                            } else if (c > pc) {
                                path.push_back(Operation(pr, pc, OperationType::RIGHT));
                            } else if (c < pc) {
                                path.push_back(Operation(pr, pc, OperationType::LEFT));
                            }
                            current = {pr, pc};
                            moves++;
                        }

                        for (int i = path.size() - 1; i >= 0; i--) {
                            operations.push_back(path[i]);
                        }

                        board[goal.r][goal.c] = board[i][j];
                        board[i][j] = 0;
                    }
                }
            }

            int misplaced = 0;
            int mismatched = 0;
            for (int i = 0; i < input.N; i++) {
                for (int j = 0; j < input.N; j++) {
                    if (board[i][j] > 0 && input.TARGET[i][j] == 0) {
                        misplaced++;
                        continue;
                    }
                    if (board[i][j] > 0 && board[i][j] != input.TARGET[i][j]) {
                        mismatched++;
                        continue;
                    }
                }
            }

            int score = moves + places * input.K + misplaced * input.N + (int)((double)mismatched * (double)input.N / 2.0);
            cerr << "score: " << score << endl;

            if (score < best_score) {
                best_score = score;
                best_operations = operations;
            }

            iter++;
        }

        cerr << "iter: " << iter << endl;
        cerr << "best score: " << best_score << endl;

        return {best_operations.size(), best_operations};
    }

    Output beam_search() {
        struct BeamNode {
            vector<Operation> operations;
            int8_t board[900];
            int moves;
            int places;
            int misplaced;
            int mismatched;
            int score;

            bool operator<(const BeamNode &other) const {
                return score < other.score;
            }
        };

        random_device seed_gen;
        mt19937 engine(seed_gen());

        int initial_moves = 0;
        int initial_places = 0;
        int initial_mismatched = 0;
        int initial_misplaced = 0;
        int WALL = -1;
        int initial_board[900];
        for (int i = 0; i < input.N; i++) {
            for (int j = 0; j < input.N; j++) {
                initial_board[i * input.N + j] = input.START[i][j];
                if (input.START[i][j] > 0 && input.TARGET[i][j] == 0) {
                    initial_misplaced++;  // ターゲットがない場所に置かれている
                    continue;
                }
                if (input.START[i][j] > 0 && input.START[i][j] != input.TARGET[i][j]) {
                    initial_mismatched++;  // ターゲットと異なる色が置かれている
                    continue;
                }
            }
        }
        auto evaluate = [&](BeamNode node) {
            return node.moves + node.places * input.K + node.misplaced * input.N + (int)((double)node.mismatched * (double)input.N / 2.0);
        };
        BeamNode initial_beam = {
            .operations = {},
            .moves = initial_moves,
            .places = initial_places,
            .misplaced = initial_misplaced,
            .mismatched = initial_mismatched,
        };
        std::copy(initial_board, initial_board + 900, initial_beam.board);
        initial_beam.score = evaluate(initial_beam) + engine() % input.N;
        // {
        //     int place_r = 3, place_c = 6;
        //     initial_beam.board[place_r * input.N + place_c] = WALL;
        //     initial_beam.places++;
        //     initial_beam.operations.push_back(Operation(place_r, place_c, OperationType::PLACE));
        // }
        BeamNode best_beam = initial_beam;
        chrono::steady_clock::time_point begin = chrono::steady_clock::now();
        int tl = 9500;
        int beam_width = 1;
        vector<Point> voids;
        for (int i = 0; i < input.N; i++) {
            for (int j = 0; j < input.N; j++) {
                if (input.TARGET[i][j] == 0 && input.START[i][j] == 0) {
                    voids.push_back(Point(i, j));
                }
            }
        }

        while (chrono::steady_clock::now() - begin < chrono::milliseconds(tl)) {
            cerr << "beam_width: " << beam_width << endl;

            vector<BeamNode> beams = {initial_beam};
            // beam_width−1個の壁をおいたinitial_beamを作る
            for (int i = 0; i < beam_width - 1; i++) {
                BeamNode beam = initial_beam;
                shuffle(voids.begin(), voids.end(), engine);
                int place_count = engine() % (voids.size() / 4 + 2);
                for (int j = 0; j < min(place_count, (int)voids.size()); j++) {
                    Point p = voids[j];
                    beam.board[p.r * input.N + p.c] = WALL;
                    beam.places++;
                    beam.operations.push_back(Operation(p.r, p.c, OperationType::PLACE));
                }
                beam.score = evaluate(beam) + engine() % input.N;
                beams.push_back(beam);
            }
            BeamNode local_best_beam = initial_beam;
            int iter = 1;
            while (chrono::steady_clock::now() - begin < chrono::milliseconds(tl)) {
                cerr << "iter: " << iter << endl;
                cerr << "best score: " << local_best_beam.score << " / " << best_beam.score << endl;
                // vector<BeamNode> next_beams;
                priority_queue<BeamNode> next_beams;
                for (BeamNode beam : beams) {
                    for (int i = 0; i < input.N; i++) {
                        for (int j = 0; j < input.N; j++) {
                            if (beam.board[i * input.N + j] > 0 && beam.board[i * input.N + j] != input.TARGET[i][j]) {
                                // for (OperationType direction : DIRECTIONS) {
                                //     auto [dr, dc] = getDirection(direction);
                                //     int r = i;
                                //     int c = j;
                                //     while (r + dr >= 0 && r + dr < input.N && c + dc >= 0 && c + dc < input.N && ((r + dr == i && c + dc == j) || beam.board[(r + dr) * input.N + (c + dc)] == 0)) {
                                //         r += dr;
                                //         c += dc;
                                //     }
                                //     if (r == i && c == j) {
                                //         continue;
                                //     }

                                //     BeamNode next_beam = beam;
                                //     next_beam.moves++;
                                //     next_beam.operations.push_back(Operation(i, j, direction));
                                //     next_beam.board[r * input.N + c] = beam.board[i * input.N + j];
                                //     next_beam.board[i * input.N + j] = 0;

                                //     // もともとターゲットがない場所に置かれていた色が、ターゲットがある場所に置かれた場合
                                //     if (input.TARGET[i][j] == 0 && input.TARGET[r][c] > 0) {
                                //         next_beam.misplaced--;
                                //     }
                                //     // もともとターゲットがある場所に置かれていた色が、ターゲットがない場所に置かれた場合
                                //     if (input.TARGET[i][j] > 0 && input.TARGET[r][c] == 0) {
                                //         next_beam.misplaced++;
                                //     }
                                //     // もともと誤った色が置かれていた場所が移動してなくなった場合
                                //     if (input.TARGET[i][j] > 0 && input.TARGET[i][j] != beam.board[i * input.N + j]) {
                                //         next_beam.mismatched--;
                                //     }
                                //     // 移動先が誤った色になった場合
                                //     if (input.TARGET[r][c] > 0 && input.TARGET[r][c] != next_beam.board[r * input.N + c]) {
                                //         next_beam.mismatched++;
                                //     }

                                //     next_beam.score = evaluate(next_beam);
                                //     next_beams.push(next_beam);
                                //     if (next_beams.size() > beam_width) {
                                //         next_beams.pop();
                                //     }
                                // }

                                int target_c = beam.board[i * input.N + j];
                                queue<pair<Point, OperationType>> q;
                                q.push({Point(i, j), PLACE});
                                map<Point, Point> prev;
                                vector<Point> goals;

                                while (!q.empty()) {
                                    auto [current_point, prev_dir] = q.front();
                                    auto [r, c] = current_point;
                                    q.pop();

                                    if (input.TARGET[r][c] == target_c) {
                                        goals.push_back(Point(r, c));
                                    }

                                    for (OperationType direction : DIRECTIONS) {
                                        if (getOppositeDirection(direction) == prev_dir) {
                                            continue;
                                        }
                                        auto [dr, dc] = getDirection(direction);
                                        int rc = r;
                                        int cc = c;
                                        while (rc + dr >= 0 && rc + dr < input.N && cc + dc >= 0 && cc + dc < input.N && ((rc + dr == i && cc + dc == j) || beam.board[(rc + dr) * input.N + (cc + dc)] == 0)) {
                                            rc += dr;
                                            cc += dc;
                                        }
                                        if (prev.count(Point(rc, cc)) == 0) {
                                            q.push({Point(rc, cc), direction});
                                            prev[Point(rc, cc)] = Point(r, c);
                                        }
                                    }
                                }

                                if (goals.empty()) {
                                    continue;
                                }

                                for (Point goal : goals) {
                                    BeamNode next_beam = beam;
                                    Point current = goal;
                                    vector<Operation> path;
                                    while (current != Point(i, j)) {
                                        auto [r, c] = current;
                                        auto [pr, pc] = prev[current];
                                        if (r > pr) {
                                            path.push_back(Operation(pr, pc, OperationType::DOWN));
                                        } else if (r < pr) {
                                            path.push_back(Operation(pr, pc, OperationType::UP));
                                        } else if (c > pc) {
                                            path.push_back(Operation(pr, pc, OperationType::RIGHT));
                                        } else if (c < pc) {
                                            path.push_back(Operation(pr, pc, OperationType::LEFT));
                                        }
                                        current = {pr, pc};
                                        next_beam.moves++;
                                    }

                                    for (int i = path.size() - 1; i >= 0; i--) {
                                        next_beam.operations.push_back(path[i]);
                                    }

                                    next_beam.board[goal.r * input.N + goal.c] = target_c;
                                    next_beam.board[i * input.N + j] = 0;

                                    // もともとターゲットがない場所に置かれていた色が、ターゲットがある場所に置かれた場合
                                    if (input.TARGET[i][j] == 0 && input.TARGET[goal.r][goal.c] > 0) {
                                        next_beam.misplaced--;
                                    }
                                    // もともとターゲットがある場所に置かれていた色が、ターゲットがない場所に置かれた場合
                                    if (input.TARGET[i][j] > 0 && input.TARGET[goal.r][goal.c] == 0) {
                                        next_beam.misplaced++;
                                    }
                                    // もともと誤った色が置かれていた場所が移動してなくなった場合
                                    if (input.TARGET[i][j] > 0 && input.TARGET[i][j] != beam.board[i * input.N + j]) {
                                        next_beam.mismatched--;
                                    }
                                    // 移動先が誤った色になった場合
                                    if (input.TARGET[goal.r][goal.c] > 0 && input.TARGET[goal.r][goal.c] != next_beam.board[goal.r * input.N + goal.c]) {
                                        next_beam.mismatched++;
                                    }

                                    next_beam.score = evaluate(next_beam) + engine() % input.N;
                                    next_beams.push(next_beam);
                                    if (next_beams.size() > beam_width) {
                                        next_beams.pop();
                                    }
                                }
                            }
                        }
                    }
                }

                if (next_beams.empty()) {
                    break;
                }

                beams.clear();

                BeamNode best = next_beams.top();
                best.score = evaluate(best);
                while (!next_beams.empty()) {
                    BeamNode beam = next_beams.top();
                    beams.push_back(beam);
                    beam.score = evaluate(beam);
                    if (beam.score < best.score) {
                        best = beam;
                    }
                    next_beams.pop();
                }

                if (best.score < local_best_beam.score) {
                    local_best_beam = best;
                }

                iter++;
            }

            if (local_best_beam.score < best_beam.score) {
                cerr << "update best score: " << local_best_beam.score << endl;
                best_beam = local_best_beam;
            }

            beam_width *= 2;
        }

        return {best_beam.operations.size(), best_beam.operations};
    }

    Output solve() {
        return greedy();
        // return beam_search();
    }
};
}  // namespace solver

using namespace io;
using namespace solver;

int main() {
    Input input;
    Solver solver(input);
    Output output = solver.solve();
    output.print();

    return 0;
}

#include <cstdio>
#include <cmath>
#include <cstring>
#include <ctime>
#include <iostream>
#include <algorithm>
#include <set>
#include <vector>
#include <sstream>
#include <typeinfo>
#include <fstream>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;

class PartialRaceResults {
    public:
    string reconstruct(vector<string> mem) {
      vector<pair<char, char>> t;
      for (string m: mem){
        t.push_back(make_pair(m[2], m[0]));
        t.push_back(make_pair(m[0], m[3]));
      }
      vector<vector<int>> g(256);
      vector<int> indeg(256);
      vector<bool> pres(256);
      for (auto p: t) {
        g[p.second].push_back(p.first);
        pres[p.first] = 1;
        pres[p.second] = 1;
        indeg[p.first]++;
      }
      vector<int> que;
      REP(i, 0, 256) {
        if (indeg[i] == 0) que.push_back(i);
      }
      string s = "";
      int rem = 256;
      while (que.size()) {
        int v = que.back(); que.pop_back();
        rem--;
        if (pres[v]) {
          s += (char) v;
        }
        for (int w: g[v]) {
          indeg[w]--;
          if (indeg[w] == 0) {
            que.push_back(w);
          }
        }
      }
      reverse(s.begin(), s.end());
      return rem == 0 ? s : "";
    }
};

// CUT begin
ifstream data("PartialRaceResults.sample");

string next_line() {
    string s;
    getline(data, s);
    return s;
}

template <typename T> void from_stream(T &t) {
    stringstream ss(next_line());
    ss >> t;
}

void from_stream(string &s) {
    s = next_line();
}

template <typename T> void from_stream(vector<T> &ts) {
    int len;
    from_stream(len);
    ts.clear();
    for (int i = 0; i < len; ++i) {
        T t;
        from_stream(t);
        ts.push_back(t);
    }
}

template <typename T>
string to_string(T t) {
    stringstream s;
    s << t;
    return s.str();
}

string to_string(string t) {
    return "\"" + t + "\"";
}

bool do_test(vector<string> memories, string __expected) {
    time_t startClock = clock();
    PartialRaceResults *instance = new PartialRaceResults();
    string __result = instance->reconstruct(memories);
    double elapsed = (double)(clock() - startClock) / CLOCKS_PER_SEC;
    delete instance;

    if (__result == __expected) {
        cout << "PASSED!" << " (" << elapsed << " seconds)" << endl;
        return true;
    }
    else {
        cout << "FAILED!" << " (" << elapsed << " seconds)" << endl;
        cout << "           Expected: " << to_string(__expected) << endl;
        cout << "           Received: " << to_string(__result) << endl;
        return false;
    }
}

int run_test(bool mainProcess, const set<int> &case_set, const string command) {
    int cases = 0, passed = 0;
    while (true) {
        if (next_line().find("--") != 0)
            break;
        vector<string> memories;
        from_stream(memories);
        next_line();
        string __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(memories, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1631891104;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 250 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
    }
    return 0;
}

int main(int argc, char *argv[]) {
    cout.setf(ios::fixed, ios::floatfield);
    cout.precision(2);
    set<int> cases;
    bool mainProcess = true;
    for (int i = 1; i < argc; ++i) {
        if ( string(argv[i]) == "-") {
            mainProcess = false;
        } else {
            cases.insert(atoi(argv[i]));
        }
    }
    if (mainProcess) {
        cout << "PartialRaceResults (250 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end

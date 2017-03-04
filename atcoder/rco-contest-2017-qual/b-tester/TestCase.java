import java.security.NoSuchAlgorithmException;
import java.security.SecureRandom;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Scanner;

// テストケース生成とシミュレーション
class TestCase {
	static final int MIN_H = 50;
	static final int MAX_H = 50;
	static final int MIN_W = 50;
	static final int MAX_W = 50;
	static final double MIN_K_RATIO = 1;
	static final double MAX_K_RATIO = 1;
	static final double MIN_N_RATIO = 0.1;
	static final double MAX_N_RATIO = 0.8;
	static final double MIN_DANCE_RATIO = 1.0;
	static final double MAX_DANCE_RATIO = 1.5;
	static final int MIN_F = 0;
	static final int MAX_F = 100000;
	static final int MIN_D_BASE = 0;
	static final int MAX_D_BASE = 100*50*50;
	
	SecureRandom rnd;
	int H, W, K, N, SR, SC;
	char[][] map;
	List<Food> foods;
	
	static final String DIRS = "UDLR-";
	static final int[] dr = { -1, 1, 0, 0, 0 };
	static final int[] dc = { 0, 0, -1, 1, 0 };

	// seedからテストケースを生成
	TestCase(long seed) {
		try {
			rnd = SecureRandom.getInstance("SHA1PRNG");
		} catch (NoSuchAlgorithmException e) {
			e.printStackTrace();
		}
		rnd.setSeed(seed);
		
		// マップサイズ
		H = getRandomInt(MIN_H, MAX_H);
		W = getRandomInt(MIN_W, MAX_W);
		
		// ステップ数上限
		K = getRandomInt((int)Math.round(H*W*MIN_K_RATIO), (int)Math.round(H*W*MAX_K_RATIO));
		
		// マップ作成
		map = new char[H][W];
		for(int i = 0;i < H;i++){
			Arrays.fill(map[i], '#');
		}
		
		// マップ中央からstep回ランダムウォークする。
		// 移動する前に、1/3の確率で方向をランダムに変える。
		// 移動の結果端に到達したら中央にワープする。
		{
			int dsr = H/2, dsc = W/2;
			int r = dsr, c = dsc;
			int step = getRandomInt((int)Math.round(H*W*MIN_DANCE_RATIO), (int)Math.round(H*W*MAX_DANCE_RATIO));
			int dir = (rnd.nextInt(H+W) < H ? 0 : 2)|rnd.nextInt(2);
			for(int i = 0;i < step;i++){
				map[r][c] = '.';
				if(rnd.nextInt(3) == 0){
					dir = (rnd.nextInt(H+W) < H ? 0 : 2)|rnd.nextInt(2);
				}
				int nr = r + dr[dir];
				int nc = c + dc[dir];
				if(nr >= 0+1 && nr < H-1 && nc >= 0+1 && nc < W-1){
					r = nr;
					c = nc;
				}else{
					r = dsr;
					c = dsc;
				}
			}
		}
		
		// 空マス列挙
		List<int[]> validCells = new ArrayList<>();
		for(int i = 0;i < H;i++){
			for(int j = 0;j < W;j++){
				if(map[i][j] == '.'){
					validCells.add(new int[]{i, j});
				}
			}
		}
		
		// 開始位置
		int[] S = validCells.remove(rnd.nextInt(validCells.size()));
		SR = S[0]; SC = S[1];
		
		// エサ
		N = getRandomInt((int)Math.round(validCells.size()*MIN_N_RATIO), (int)Math.round(validCells.size()*MAX_N_RATIO));
		Collections.shuffle(validCells, rnd);
		foods = new ArrayList<>();
		for(int i = 0;i < N;i++){
			foods.add(new Food(validCells.get(i)[0], validCells.get(i)[1], getRandomInt(MIN_F, MAX_F), getRandomInt(MIN_D_BASE/(H*W), MAX_D_BASE/(H*W))));
		}
	}
	
	// 入力文字列からテストケースを生成
	TestCase(String input) {
		try(Scanner in = new Scanner(input)){
			// マップサイズ
			H = in.nextInt();
			W = in.nextInt();
		
			// ステップ数上限
			K = in.nextInt();
		
			// 開始位置
			SR = in.nextInt() - 1;
			SC = in.nextInt() - 1;
			
			// マップ作成
			map = new char[H][];
			for(int i = 0;i < H;i++){
				map[i] = in.next().toCharArray();
			}
		
			// エサ
			N = in.nextInt();
			foods = new ArrayList<>();
			for(int i = 0;i < N;i++){
				foods.add(new Food(in.nextInt()-1, in.nextInt()-1, in.nextInt(), in.nextInt()));
			}
		}
	}
	
	// 経路をシミュレートしてスコアを出力する
	public long simulate(char[] route) {
		if(route.length != K)throw new IllegalArgumentException();
		for(char c : route)if(DIRS.indexOf(c) == -1)throw new IllegalArgumentException();
		
		Food[][] foodmap = new Food[H][W];
		for(Food f : foods)foodmap[f.r][f.c] = f;
		
		long score = 0;
		int[] pos = {SR, SC};
		for(int turn = 0;turn < K;turn++){
			int dir = DIRS.indexOf(route[turn]);
			int nr = pos[0]+dr[dir];
			int nc = pos[1]+dc[dir];
			if(map[nr][nc] == '#'){
				// 移動失敗
			}else{
				// 移動成功
				pos[0] = nr;
				pos[1] = nc;
				// エサがあれば強制獲得
				if(foodmap[nr][nc] != null){
					Food food = foodmap[nr][nc];
					foodmap[nr][nc] = null;
					score += food.f - (long)turn*food.d;
				}
			}
		}
		
		return score;
	}
	
	public long toTruncatedScore(long originalScore) {
		// 0以下のスコアは0に、正のスコアは10000で割って切り上げ
		return originalScore <= 0 ? 0 : (originalScore+9999)/10000;
	}
	
	public String mapToString() {
		char[][] xmap = new char[H][];
		for(int i = 0;i < H;i++)xmap[i] = Arrays.copyOf(map[i], W);
		
		// スタート地点
		xmap[SR][SC] = 'S';
		
		// エサ
		for(Food f : foods){
			xmap[f.r][f.c] = 'f';
		}
		
		StringBuilder sb = new StringBuilder();
		for(char[] row : xmap){
			sb.append(new String(row)).append("\n");
		}
		return sb.toString();
	}
	
	// 移動した軌跡を'o'で表したマップを返す
	public String trailMapToString(char[] route) {
		if(route.length != K)throw new IllegalArgumentException();
		for(char c : route)if(DIRS.indexOf(c) == -1)throw new IllegalArgumentException();
		
		char[][] xmap = new char[H][];
		for(int i = 0;i < H;i++)xmap[i] = Arrays.copyOf(map[i], W);
		
		// エサ
		for(Food f : foods){
			xmap[f.r][f.c] = 'f';
		}
		
		Food[][] foodmap = new Food[H][W];
		for(Food f : foods)foodmap[f.r][f.c] = f;
		
		int[] pos = {SR, SC};
		for(int turn = 0;turn < K;turn++){
			int dir = DIRS.indexOf(route[turn]);
			int nr = pos[0]+dr[dir];
			int nc = pos[1]+dc[dir];
			if(map[nr][nc] == '#'){
				// 移動失敗
			}else{
				// 移動成功
				pos[0] = nr;
				pos[1] = nc;
				xmap[nr][nc] = 'o';
			}
		}
		
		// スタート地点
		xmap[SR][SC] = 'S';
		
		StringBuilder sb = new StringBuilder();
		for(char[] row : xmap){
			sb.append(new String(row)).append("\n");
		}
		return sb.toString();
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();
		sb.append(H + " " + W + " " + K + " " + (SR+1) + " " + (SC+1)).append("\n");
		for(char[] row : map){
			sb.append(new String(row)).append("\n");
		}
		sb.append(N).append("\n");
		for(Food food : foods){
			sb.append(food).append("\n");
		}
		return sb.toString();
	}
	
	// [minInclusive, maxInclusive]から等確率ランダムに選ぶ
	private int getRandomInt(int minInclusive, int maxInclusive) {
		return rnd.nextInt(maxInclusive - minInclusive + 1) + minInclusive;
	}
	
	// エサ
	static class Food {
		public int r, c;
		public int f, d;
		public Food(int r, int c, int f, int d) {
			this.r = r;
			this.c = c;
			this.f = f;
			this.d = d;
		}
		
		@Override
		public String toString() {
			return (r+1) + " " + (c+1) + " " + f + " " + d;
		}
	}
}
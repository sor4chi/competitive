interface Config {
  title: string;
  description?: string;
  link?: {
    href: string;
    text: string;
  };
}

export const CONFIG: Config = {
  title: '第一回マスターズ選手権 予選 ビジュアライザ',
  description:
    'AtCoderで 2024-03-03(日) 13:00 ~ 2024-03-03(日) 19:00 に開催された「第一回マスターズ選手権-予選-」のビジュアライザです。',
  link: {
    href: 'https://atcoder.jp/contests/masters-qual',
    text: '問題ページへ',
  },
};
